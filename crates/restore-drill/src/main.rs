mod config;
mod docker;
mod report;

use chrono::Utc;
use clap::{Parser, Subcommand};
use config::{Config, DEFAULT_CONFIG, DEFAULT_ENV, SourceKind};
use docker::{Docker, DrillRun, hash_file};
use report::{ArtifactEvidence, Report};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::{Arc, atomic::AtomicBool};
use std::time::{Duration, Instant};

#[derive(Parser)]
#[command(name = "restore-drill", version, about = "Prove a Postgres backup restores in an isolated Docker network", long_about = None)]
struct Cli {
    /// Docker CLI path (useful for rootless or test installations)
    #[arg(
        long,
        global = true,
        env = "RESTORE_DRILL_DOCKER",
        default_value = "docker"
    )]
    docker: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Write a safe starter config and local credential file
    Init {
        #[arg(short, long, default_value = "restore-drill.toml")]
        output: PathBuf,
        /// Replace an existing starter config (credential files are never replaced)
        #[arg(long)]
        force: bool,
    },
    /// Validate config, safety boundaries, files, and Docker access
    Check {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Restore, probe, record signed evidence, and tear down
    Run {
        #[arg(short, long, default_value = "restore-drill.toml")]
        config: PathBuf,
        /// Emit only the final report JSON to stdout
        #[arg(long)]
        json: bool,
        /// Keep isolated resources after a failed drill for local diagnosis
        #[arg(long)]
        keep_on_failure: bool,
    },
    /// Verify that a report has not been altered
    Verify {
        report: PathBuf,
        /// Compare the embedded signer with a separately retained public key
        #[arg(long)]
        public_key: Option<PathBuf>,
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match real_main() {
        Ok(code) => ExitCode::from(code),
        Err((code, message)) => {
            eprintln!("restore-drill: {message}");
            ExitCode::from(code)
        }
    }
}

fn real_main() -> Result<u8, (u8, String)> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { output, force } => init(&output, force).map(|_| 0).map_err(|e| (2, e)),
        Commands::Check { config, json } => check(&cli.docker, &config, json)
            .map(|_| 0)
            .map_err(|e| (2, e)),
        Commands::Run {
            config,
            json,
            keep_on_failure,
        } => run(&cli.docker, &config, json, keep_on_failure),
        Commands::Verify {
            report,
            public_key,
            json,
        } => verify(&report, public_key.as_deref(), json)
            .map(|_| 0)
            .map_err(|e| (1, e)),
    }
}

fn init(output: &Path, force: bool) -> Result<(), String> {
    if output.exists() && !force {
        return Err(format!(
            "{} already exists; use --force to replace only the config",
            output.display()
        ));
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("could not create config directory: {e}"))?;
    }
    fs::write(output, DEFAULT_CONFIG)
        .map_err(|e| format!("could not write {}: {e}", output.display()))?;
    let env = output
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(".restore-drill.env");
    if !env.exists() {
        let mut options = fs::OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        use std::io::Write;
        options
            .open(&env)
            .and_then(|mut file| file.write_all(DEFAULT_ENV.as_bytes()))
            .map_err(|e| format!("could not write {}: {e}", env.display()))?;
    }
    println!("Wrote {} and {}", output.display(), env.display());
    println!(
        "Next: set POSTGRES_PASSWORD, choose the backup path, then run restore-drill check --config {}",
        output.display()
    );
    Ok(())
}

fn check(docker_path: &Path, path: &Path, json: bool) -> Result<(), String> {
    let config = Config::load(path)?;
    Docker::new(docker_path.to_path_buf(), Duration::from_secs(20)).check()?;
    if json {
        println!(
            "{}",
            serde_json::json!({"valid":true,"drill":config.drill.name,"network":config.drill.network,"source":config.source.path})
        );
    } else {
        println!(
            "✓ config and safety boundary valid\n✓ backup and credential files readable\n✓ Docker engine reachable"
        );
    }
    Ok(())
}

fn run(
    docker_path: &Path,
    path: &Path,
    json: bool,
    keep_on_failure: bool,
) -> Result<u8, (u8, String)> {
    let config = Config::load(path).map_err(|e| (2, e))?;
    let docker = Docker::new(
        docker_path.to_path_buf(),
        Duration::from_secs(config.drill.timeout_seconds),
    );
    docker.check().map_err(|e| (2, e))?;
    let (sha256, bytes) = hash_file(&config.source.path).map_err(|e| (2, e))?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let signal = cancelled.clone();
    ctrlc::set_handler(move || signal.store(true, std::sync::atomic::Ordering::SeqCst))
        .map_err(|e| (2, format!("could not install signal handler: {e}")))?;
    let started_at = Utc::now();
    let timer = Instant::now();
    if !json {
        eprintln!("restore-drill: hashing complete; preparing isolated images");
    }
    let mut drill = DrillRun::new(&docker, &config, cancelled);
    let mut images = BTreeMap::new();
    let mut assertions = vec![];
    let outcome: Result<(), String> = (|| {
        images = drill.prepare_images()?;
        if !json {
            eprintln!(
                "restore-drill: creating internal network {}",
                config.drill.network
            );
        }
        drill.create_environment()?;
        if !json {
            eprintln!(
                "restore-drill: restore complete; running {} assertions",
                config.assertions.sql.len() + config.assertions.http.len()
            );
        }
        assertions = drill.assertions();
        if assertions.iter().any(|a| !a.passed) {
            return Err("one or more smoke assertions failed".into());
        }
        Ok(())
    })();
    let mut error = outcome.err();
    if error.is_some() && keep_on_failure {
        drill.retain();
        let resources = drill.resources();
        eprintln!("restore-drill: keeping isolated resources: {resources}");
        error = error.map(|message| format!("{message}; isolated resources retained: {resources}"));
    }
    let cleanup_errors = drill.cleanup();
    if !cleanup_errors.is_empty() {
        let cleanup = format!("cleanup was incomplete: {}", cleanup_errors.join("; "));
        error = Some(error.map_or(cleanup.clone(), |message| format!("{message}; {cleanup}")));
    }
    match hash_file(&config.source.path) {
        Ok((after_hash, after_bytes)) if after_hash != sha256 || after_bytes != bytes => {
            let changed = "backup source changed while the drill was running".to_string();
            error = Some(error.map_or(changed.clone(), |message| format!("{message}; {changed}")));
        }
        Err(hash_error) => {
            let changed = format!("could not re-check backup after the drill: {hash_error}");
            error = Some(error.map_or(changed.clone(), |message| format!("{message}; {changed}")));
        }
        _ => {}
    }
    let finished_at = Utc::now();
    let mut report = Report {
        schema: "restore-drill/v1".into(),
        drill: config.drill.name.clone(),
        started_at,
        finished_at,
        recovery_time_ms: timer.elapsed().as_millis() as u64,
        status: if error.is_none() {
            "passed".into()
        } else {
            "failed".into()
        },
        artifact: ArtifactEvidence {
            file_name: config
                .source
                .path
                .file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("backup")
                .into(),
            kind: match config.source.kind {
                SourceKind::Dump => "dump",
                SourceKind::VolumeTar => "volume_tar",
            }
            .into(),
            sha256,
            bytes,
        },
        images,
        assertions,
        error,
        public_key: String::new(),
        signature: String::new(),
    };
    let report_path = report
        .sign_and_write(&config.drill.signing_key, &config.drill.report_dir)
        .map_err(|e| (1, e))?;
    if json {
        println!(
            "{}",
            serde_json::to_string(&report).map_err(|e| (1, e.to_string()))?
        );
    } else {
        let mark = if report.status == "passed" {
            "✓"
        } else {
            "✗"
        };
        eprintln!(
            "{mark} {} in {:.2}s — report {}",
            report.status,
            report.recovery_time_ms as f64 / 1000.0,
            report_path.display()
        );
        for a in &report.assertions {
            eprintln!(
                "  {} {}: {}",
                if a.passed { "✓" } else { "✗" },
                a.name,
                a.observed
            );
        }
    }
    Ok(if report.status == "passed" { 0 } else { 1 })
}

fn verify(path: &Path, public_key: Option<&Path>, json: bool) -> Result<(), String> {
    let report = report::read_and_verify(path)?;
    if let Some(path) = public_key {
        let expected = fs::read_to_string(path)
            .map_err(|e| format!("could not read public key {}: {e}", path.display()))?;
        if expected.trim() != report.public_key {
            return Err("report was signed by a different key".into());
        }
    }
    if json {
        println!(
            "{}",
            serde_json::json!({"valid":true,"drill":report.drill,"status":report.status,"finished_at":report.finished_at})
        );
    } else {
        println!(
            "✓ signature valid — {} was {} at {}",
            report.drill,
            report.status,
            report.finished_at.to_rfc3339()
        );
    }
    Ok(())
}
