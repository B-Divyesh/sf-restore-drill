//! Real-engine acceptance tests. These are ignored by the default suite because
//! they pull Postgres and require a Docker daemon. The claim workflow runs them.

use assert_cmd::Command;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn docker(args: &[&str]) -> String {
    let output = std::process::Command::new("docker")
        .args(args)
        .output()
        .expect("Docker CLI must be installed for this ignored acceptance test");
    assert!(
        output.status.success(),
        "docker {} failed: {}",
        args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

fn assert_no_managed_resources() {
    for args in [
        &[
            "ps",
            "-aq",
            "--filter",
            "label=in.sociobot.restore-drill=managed",
        ][..],
        &[
            "network",
            "ls",
            "-q",
            "--filter",
            "label=in.sociobot.restore-drill=managed",
        ][..],
        &[
            "volume",
            "ls",
            "-q",
            "--filter",
            "label=in.sociobot.restore-drill=managed",
        ][..],
    ] {
        assert!(
            docker(args).is_empty(),
            "managed Docker resources remained after the drill"
        );
    }
}

fn only_report_in(root: &Path) -> PathBuf {
    let reports = root.join("reports");
    let paths: Vec<_> = fs::read_dir(reports)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    assert_eq!(paths.len(), 1, "each acceptance run must write one report");
    paths[0].clone()
}

// @claim:real-docker-restore
#[test]
#[ignore = "requires a real Docker daemon and pulls postgres:16-alpine"]
fn real_docker_demo_restores_and_corrupt_dump_fails_with_cleanup() {
    docker(&["version"]);
    assert_no_managed_resources();

    let demo_parent = tempfile::tempdir().unwrap();
    let healthy = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("demo")
        .env("TMPDIR", demo_parent.path())
        .output()
        .unwrap();
    assert!(
        healthy.status.success(),
        "{}",
        String::from_utf8_lossy(&healthy.stderr)
    );
    let demo_root = fs::read_dir(demo_parent.path())
        .unwrap()
        .map(|entry| entry.unwrap())
        .find(|entry| {
            entry.file_type().is_ok_and(|kind| kind.is_dir())
                && entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with("restore-drill-demo-")
        })
        .expect("demo command must create its isolated temporary directory")
        .path();
    let healthy_path = only_report_in(&demo_root);
    let healthy_report: Value = serde_json::from_slice(&fs::read(&healthy_path).unwrap()).unwrap();
    assert_eq!(healthy_report["status"], "passed");
    assert_eq!(healthy_report["assertions"][0]["observed"], "3");
    assert!(healthy_report["signature"].as_str().unwrap().len() > 40);
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["verify"])
        .arg(&healthy_path)
        .arg("--public-key")
        .arg(demo_root.join("signing.key.pub"))
        .assert()
        .success();
    assert_no_managed_resources();

    if let Ok(capture_dir) = std::env::var("RESTORE_DRILL_CAPTURE_DIR") {
        let capture_dir = PathBuf::from(capture_dir);
        fs::create_dir_all(&capture_dir).unwrap();
        fs::copy(&healthy_path, capture_dir.join("sample-report.json")).unwrap();
        let root_text = demo_root.display().to_string();
        let frames: Vec<Value> = String::from_utf8_lossy(&healthy.stderr)
            .lines()
            .map(|line| {
                let mut text = line.replace(&root_text, "/tmp/restore-drill-demo-…");
                if let (Some(start), Some(end)) = (text.find(" in "), text.find(" — report ")) {
                    text.replace_range(start + 4..end, "<measured time>");
                }
                serde_json::json!({"text": text, "delay_ms": 420})
            })
            .collect();
        let recording = serde_json::json!({
            "command": "restore-drill demo",
            "source": std::env::var("GITHUB_SHA").unwrap_or_else(|_| "local-real-docker-run".into()),
            "captured_at": healthy_report["finished_at"],
            "frames": frames
        });
        fs::write(
            capture_dir.join("demo-recording.json"),
            format!("{}\n", serde_json::to_string_pretty(&recording).unwrap()),
        )
        .unwrap();
    }

    let broken_root = tempfile::tempdir().unwrap();
    fs::write(
        broken_root.path().join("broken.sql"),
        "this is not valid SQL;\n",
    )
    .unwrap();
    let credentials = broken_root.path().join("demo.env");
    fs::write(&credentials, "POSTGRES_PASSWORD=restore-drill-demo-only\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&credentials, fs::Permissions::from_mode(0o600)).unwrap();
    }
    fs::write(
        broken_root.path().join("restore-drill.toml"),
        r#"version = 1
[drill]
name = "corrupt-sample"
network = "restore-drill-corrupt-sample"
timeout_seconds = 90
report_dir = "reports"
signing_key = "signing.key"
[postgres]
image = "postgres:16-alpine"
container = "restore-drill-corrupt-db"
database = "sample_app"
user = "restore_drill"
credential_file = "demo.env"
[source]
kind = "dump"
path = "broken.sql"
format = "plain"
[[assertions.sql]]
name = "three sample orders restore"
query = "SELECT count(*) FROM drill_orders"
expect = "3"
"#,
    )
    .unwrap();
    let broken = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["run", "--config"])
        .arg(broken_root.path().join("restore-drill.toml"))
        .arg("--json")
        .output()
        .unwrap();
    assert_eq!(broken.status.code(), Some(1));
    let broken_report: Value = serde_json::from_slice(&broken.stdout).unwrap();
    assert_eq!(broken_report["status"], "failed");
    assert!(
        broken_report["error"]
            .as_str()
            .unwrap()
            .contains("Docker command failed")
    );
    let broken_path = only_report_in(broken_root.path());
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["verify"])
        .arg(&broken_path)
        .arg("--public-key")
        .arg(broken_root.path().join("signing.key.pub"))
        .assert()
        .success();
    assert_no_managed_resources();
}
