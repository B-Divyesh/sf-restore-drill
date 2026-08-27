use crate::config::{Config, DumpFormat, SourceKind};
use crate::report::Evidence;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::{Duration, Instant};
use wait_timeout::ChildExt;

const LABEL: &str = "in.sociobot.restore-drill=managed";
const CURL_IMAGE: &str = "curlimages/curl:8.10.1";
const ALPINE_IMAGE: &str = "alpine:3.20";

pub struct Docker {
    binary: PathBuf,
    timeout: Duration,
}

impl Docker {
    pub fn new(binary: PathBuf, timeout: Duration) -> Self {
        Self { binary, timeout }
    }

    pub fn check(&self) -> Result<(), String> {
        self.run(&["version", "--format", "{{.Server.Version}}"])?;
        Ok(())
    }

    fn run(&self, args: &[&str]) -> Result<Output, String> {
        let args: Vec<OsString> = args.iter().map(OsString::from).collect();
        self.run_os(&args)
    }

    fn run_owned(&self, args: &[String]) -> Result<Output, String> {
        let args: Vec<OsString> = args.iter().map(OsString::from).collect();
        self.run_os(&args)
    }

    fn run_os(&self, args: &[OsString]) -> Result<Output, String> {
        let mut child = Command::new(&self.binary)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                format!(
                    "could not start Docker CLI '{}': {e}",
                    self.binary.display()
                )
            })?;
        if child
            .wait_timeout(self.timeout)
            .map_err(|e| format!("could not wait for Docker: {e}"))?
            .is_none()
        {
            let _ = child.kill();
            let _ = child.wait();
            return Err(format!(
                "Docker command timed out after {} seconds",
                self.timeout.as_secs()
            ));
        }
        let output = child
            .wait_with_output()
            .map_err(|e| format!("could not read Docker output: {e}"))?;
        if output.status.success() {
            Ok(output)
        } else {
            let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(if detail.is_empty() {
                "Docker command failed".into()
            } else {
                format!("Docker command failed: {detail}")
            })
        }
    }

    fn ensure_image(&self, image: &str) -> Result<String, String> {
        if self
            .run_owned(&[
                "image".into(),
                "inspect".into(),
                "--format".into(),
                "{{.Id}}".into(),
                image.into(),
            ])
            .is_err()
        {
            self.run_owned(&["pull".into(), image.into()])?;
        }
        let out = self.run_owned(&[
            "image".into(),
            "inspect".into(),
            "--format".into(),
            "{{.Id}}".into(),
            image.into(),
        ])?;
        Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
    }
}

pub struct DrillRun<'a> {
    docker: &'a Docker,
    config: &'a Config,
    containers: Vec<String>,
    volume: Option<String>,
    network: Option<String>,
    cancelled: Arc<AtomicBool>,
    keep: bool,
}

impl<'a> DrillRun<'a> {
    pub fn new(docker: &'a Docker, config: &'a Config, cancelled: Arc<AtomicBool>) -> Self {
        Self {
            docker,
            config,
            containers: vec![],
            volume: None,
            network: None,
            cancelled,
            keep: false,
        }
    }

    pub fn prepare_images(&self) -> Result<std::collections::BTreeMap<String, String>, String> {
        let mut images = std::collections::BTreeMap::new();
        images.insert(
            self.config.postgres.image.clone(),
            self.docker.ensure_image(&self.config.postgres.image)?,
        );
        for service in &self.config.services {
            images.insert(
                service.image.clone(),
                self.docker.ensure_image(&service.image)?,
            );
        }
        if !self.config.assertions.http.is_empty() {
            images.insert(CURL_IMAGE.into(), self.docker.ensure_image(CURL_IMAGE)?);
        }
        if matches!(self.config.source.kind, SourceKind::VolumeTar) {
            images.insert(ALPINE_IMAGE.into(), self.docker.ensure_image(ALPINE_IMAGE)?);
        }
        Ok(images)
    }

    pub fn create_environment(&mut self) -> Result<(), String> {
        self.guard_cancelled()?;
        self.docker.run_owned(&[
            "network".into(),
            "create".into(),
            "--internal".into(),
            "--label".into(),
            LABEL.into(),
            self.config.drill.network.clone(),
        ])?;
        self.network = Some(self.config.drill.network.clone());
        let volume = format!(
            "restore-drill-{}-{}",
            self.config.drill.name,
            std::process::id()
        );
        self.docker.run_owned(&[
            "volume".into(),
            "create".into(),
            "--label".into(),
            LABEL.into(),
            volume.clone(),
        ])?;
        self.volume = Some(volume.clone());
        if matches!(self.config.source.kind, SourceKind::VolumeTar) {
            self.extract_volume(&volume)?;
        }
        self.start_postgres(&volume)?;
        self.wait_for_postgres()?;
        if matches!(self.config.source.kind, SourceKind::Dump) {
            self.restore_dump()?;
        }
        for service in &self.config.services {
            self.start_service(service)?;
        }
        Ok(())
    }

    fn start_postgres(&mut self, volume: &str) -> Result<(), String> {
        let p = &self.config.postgres;
        let args = vec![
            "run".into(),
            "-d".into(),
            "--name".into(),
            p.container.clone(),
            "--network".into(),
            self.config.drill.network.clone(),
            "--network-alias".into(),
            p.container.clone(),
            "--label".into(),
            LABEL.into(),
            "--env-file".into(),
            p.credential_file.display().to_string(),
            "-e".into(),
            format!("POSTGRES_DB={}", p.database),
            "-e".into(),
            format!("POSTGRES_USER={}", p.user),
            "-v".into(),
            format!("{volume}:/var/lib/postgresql/data"),
            p.image.clone(),
        ];
        self.docker.run_owned(&args)?;
        self.containers.push(p.container.clone());
        Ok(())
    }

    fn wait_for_postgres(&self) -> Result<(), String> {
        let deadline = Instant::now() + Duration::from_secs(self.config.drill.timeout_seconds);
        loop {
            self.guard_cancelled()?;
            if self
                .docker
                .run_owned(&[
                    "exec".into(),
                    self.config.postgres.container.clone(),
                    "pg_isready".into(),
                    "-U".into(),
                    self.config.postgres.user.clone(),
                    "-d".into(),
                    self.config.postgres.database.clone(),
                ])
                .is_ok()
            {
                return Ok(());
            }
            if Instant::now() >= deadline {
                return Err("Postgres did not become ready before the drill timeout".into());
            }
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn restore_dump(&self) -> Result<(), String> {
        let source = fs::canonicalize(&self.config.source.path)
            .map_err(|e| format!("could not resolve backup path: {e}"))?;
        self.docker.run_owned(&[
            "cp".into(),
            source.display().to_string(),
            format!("{}:/tmp/restore-input", self.config.postgres.container),
        ])?;
        let format = match self.config.source.format {
            DumpFormat::Custom => DumpFormat::Custom,
            DumpFormat::Plain => DumpFormat::Plain,
            DumpFormat::Auto => {
                let header =
                    fs::read(&source).map_err(|e| format!("could not inspect backup: {e}"))?;
                if header.starts_with(b"PGDMP") {
                    DumpFormat::Custom
                } else {
                    DumpFormat::Plain
                }
            }
        };
        let p = &self.config.postgres;
        let mut args = vec!["exec".into(), p.container.clone()];
        match format {
            DumpFormat::Custom => args.extend([
                "pg_restore".into(),
                "--exit-on-error".into(),
                "--clean".into(),
                "--if-exists".into(),
                "--no-owner".into(),
                "-U".into(),
                p.user.clone(),
                "-d".into(),
                p.database.clone(),
                "/tmp/restore-input".into(),
            ]),
            _ => args.extend([
                "psql".into(),
                "-v".into(),
                "ON_ERROR_STOP=1".into(),
                "-U".into(),
                p.user.clone(),
                "-d".into(),
                p.database.clone(),
                "-f".into(),
                "/tmp/restore-input".into(),
            ]),
        }
        self.docker.run_owned(&args)?;
        let _ = self.docker.run_owned(&[
            "exec".into(),
            p.container.clone(),
            "rm".into(),
            "-f".into(),
            "/tmp/restore-input".into(),
        ]);
        Ok(())
    }

    fn extract_volume(&self, volume: &str) -> Result<(), String> {
        let path = fs::canonicalize(&self.config.source.path)
            .map_err(|e| format!("could not resolve volume archive: {e}"))?;
        let args = vec![
            "run".into(),
            "--rm".into(),
            "--label".into(),
            LABEL.into(),
            "-v".into(),
            format!("{}:/backup/archive:ro", path.display()),
            "-v".into(),
            format!("{volume}:/data"),
            ALPINE_IMAGE.into(),
            "tar".into(),
            "-xf".into(),
            "/backup/archive".into(),
            "-C".into(),
            "/data".into(),
        ];
        self.docker.run_owned(&args)?;
        Ok(())
    }

    fn start_service(&mut self, service: &crate::config::Service) -> Result<(), String> {
        let mut args = vec![
            "run".into(),
            "-d".into(),
            "--name".into(),
            service.name.clone(),
            "--network".into(),
            self.config.drill.network.clone(),
            "--network-alias".into(),
            service.name.clone(),
            "--label".into(),
            LABEL.into(),
        ];
        if let Some(env) = &service.env_file {
            args.extend(["--env-file".into(), env.display().to_string()]);
        }
        args.push(service.image.clone());
        args.extend(service.command.clone());
        self.docker.run_owned(&args)?;
        self.containers.push(service.name.clone());
        Ok(())
    }

    pub fn assertions(&self) -> Vec<Evidence> {
        let mut results = vec![];
        for assertion in &self.config.assertions.sql {
            let started = Instant::now();
            let out = self.docker.run_owned(&[
                "exec".into(),
                self.config.postgres.container.clone(),
                "psql".into(),
                "-A".into(),
                "-t".into(),
                "-v".into(),
                "ON_ERROR_STOP=1".into(),
                "-U".into(),
                self.config.postgres.user.clone(),
                "-d".into(),
                self.config.postgres.database.clone(),
                "-c".into(),
                assertion.query.clone(),
            ]);
            let (passed, observed) = match out {
                Ok(v) => {
                    let s = String::from_utf8_lossy(&v.stdout).trim().to_string();
                    (s == assertion.expect, s)
                }
                Err(e) => (false, e),
            };
            results.push(Evidence {
                name: assertion.name.clone(),
                kind: "sql".into(),
                passed,
                expected: assertion.expect.clone(),
                observed,
                duration_ms: started.elapsed().as_millis() as u64,
            });
        }
        for assertion in &self.config.assertions.http {
            let started = Instant::now();
            let deadline = started + Duration::from_secs(self.config.drill.timeout_seconds);
            let expected = match &assertion.body_contains {
                Some(body) => format!("status {}; body contains {:?}", assertion.status, body),
                None => format!("status {}", assertion.status),
            };
            let (passed, observed) = loop {
                if let Err(e) = self.guard_cancelled() {
                    break (false, e);
                }
                let out = self.docker.run_owned(&[
                    "run".into(),
                    "--rm".into(),
                    "--network".into(),
                    self.config.drill.network.clone(),
                    CURL_IMAGE.into(),
                    "--silent".into(),
                    "--show-error".into(),
                    "--max-time".into(),
                    "10".into(),
                    "--write-out".into(),
                    "\\n%{http_code}".into(),
                    assertion.url.clone(),
                ]);
                let observed = match out {
                    Ok(v) => {
                        let raw = String::from_utf8_lossy(&v.stdout);
                        let mut lines: Vec<&str> = raw.lines().collect();
                        let code = lines.pop().and_then(|v| v.parse::<u16>().ok()).unwrap_or(0);
                        let body = lines.join("\n");
                        let ok = code == assertion.status
                            && assertion
                                .body_contains
                                .as_ref()
                                .map(|needle| body.contains(needle))
                                .unwrap_or(true);
                        let detail = format!("status {code}; body {} bytes", body.len());
                        if ok {
                            break (true, detail);
                        }
                        detail
                    }
                    Err(e) => e,
                };
                if Instant::now() >= deadline {
                    break (false, observed);
                }
                thread::sleep(Duration::from_secs(1));
            };
            results.push(Evidence {
                name: assertion.name.clone(),
                kind: "http".into(),
                passed,
                expected,
                observed,
                duration_ms: started.elapsed().as_millis() as u64,
            });
        }
        results
    }

    pub fn retain(&mut self) {
        self.keep = true;
    }

    pub fn resources(&self) -> String {
        format!(
            "network={}, volume={}, containers={}",
            self.config.drill.network,
            self.volume.as_deref().unwrap_or("none"),
            self.containers.join(",")
        )
    }

    pub fn cleanup(&mut self) -> Vec<String> {
        if self.keep {
            return vec![];
        }
        let mut errors = vec![];
        for container in self.containers.drain(..).rev() {
            if let Err(e) = self
                .docker
                .run_owned(&["rm".into(), "-f".into(), container])
            {
                errors.push(e);
            }
        }
        if let Some(volume) = self.volume.take() {
            if let Err(e) =
                self.docker
                    .run_owned(&["volume".into(), "rm".into(), "-f".into(), volume])
            {
                errors.push(e);
            }
        }
        if let Some(network) = self.network.take() {
            if let Err(e) = self
                .docker
                .run_owned(&["network".into(), "rm".into(), network])
            {
                errors.push(e);
            }
        }
        errors
    }

    fn guard_cancelled(&self) -> Result<(), String> {
        if self.cancelled.load(Ordering::SeqCst) {
            Err("drill cancelled; disposable resources will be removed".into())
        } else {
            Ok(())
        }
    }
}

impl Drop for DrillRun<'_> {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

pub fn hash_file(path: &Path) -> Result<(String, u64), String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let mut file =
        fs::File::open(path).map_err(|e| format!("could not open backup for hashing: {e}"))?;
    let bytes = file
        .metadata()
        .map_err(|e| format!("could not inspect backup: {e}"))?
        .len();
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|e| format!("could not hash backup: {e}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok((hex::encode(hasher.finalize()), bytes))
}
