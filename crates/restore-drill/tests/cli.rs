use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::Value;
use std::fs;

#[cfg(unix)]
fn make_fake_docker(dir: &tempfile::TempDir, body: &str) -> std::path::PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let fake = dir.path().join("docker");
    fs::write(&fake, format!("#!/bin/sh\n{body}\n")).unwrap();
    fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
    fake
}

#[test]
fn help_describes_the_real_workflow() {
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("isolated Docker network"))
        .stdout(predicate::str::contains("run"))
        .stdout(predicate::str::contains("verify"));
}

#[test]
fn init_is_non_destructive() {
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("restore-drill.toml");
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["init", "--output"])
        .arg(&config)
        .assert()
        .success();
    assert!(config.exists());
    assert!(dir.path().join(".restore-drill.env").exists());
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["init", "--output"])
        .arg(&config)
        .assert()
        .code(2)
        .stderr(predicate::str::contains("already exists"));
}

#[cfg(unix)]
#[test]
fn demo_uses_shipped_sample_in_a_fresh_temporary_directory() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let fake = dir.path().join("docker");
    let log = dir.path().join("docker.log");
    fs::write(
        &fake,
        r#"#!/bin/sh
case " $* " in
  *" image inspect "*) printf 'sha256:sample-image\n' ;;
  *" exec restore-drill-sample-db psql "*" -c "*) printf '3\n' ;;
esac
printf '%s\n' "$*" >> "$FAKE_DOCKER_LOG"
exit 0
"#,
    )
    .unwrap();
    let mut permissions = fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake, permissions).unwrap();
    let output = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--docker")
        .arg(&fake)
        .args(["demo", "--json"])
        .env("FAKE_DOCKER_LOG", &log)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["drill"], "sample-orders");
    assert_eq!(report["status"], "passed");
    assert_eq!(report["artifact"]["file_name"], "sample-backup.sql");
    assert!(
        report["signature"]
            .as_str()
            .is_some_and(|value| !value.is_empty())
    );
    let events = fs::read_to_string(log).unwrap();
    assert!(events.contains("network create --internal"));
    assert!(events.contains("network rm restore-drill-sample-orders"));
}

// @claim:image-pull
#[cfg(unix)]
#[test]
fn demo_pulls_a_missing_postgres_image_before_creating_resources() {
    let dir = tempfile::tempdir().unwrap();
    let log = dir.path().join("docker.log");
    let state = dir.path().join("inspect.state");
    let fake = make_fake_docker(
        &dir,
        r#"printf '%s\n' "$*" >> "$FAKE_DOCKER_LOG"
case " $* " in
  *" image inspect "*)
    if [ ! -f "$FAKE_INSPECT_STATE" ]; then touch "$FAKE_INSPECT_STATE"; exit 1; fi
    printf 'sha256:sample-image\n' ;;
  *" exec restore-drill-sample-db psql "*" -c "*) printf '3\n' ;;
esac
exit 0"#,
    );
    let output = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--docker")
        .arg(&fake)
        .args(["demo", "--json"])
        .env("FAKE_DOCKER_LOG", &log)
        .env("FAKE_INSPECT_STATE", &state)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let events = fs::read_to_string(log).unwrap();
    let pull = events.find("pull postgres:16-alpine").unwrap();
    let create = events.find("network create --internal").unwrap();
    assert!(
        pull < create,
        "the missing image must be pulled before isolation is created"
    );
    assert_eq!(events.matches("image inspect").count(), 2);
}

// @claim:automation-contract
#[cfg(unix)]
#[test]
fn json_output_and_exit_codes_are_stable_for_automation() {
    let dir = tempfile::tempdir().unwrap();
    let fake = make_fake_docker(
        &dir,
        r#"case " $* " in
  *" image inspect "*) printf 'sha256:sample-image\n' ;;
  *" exec restore-drill-sample-db psql "*" -c "*) printf '%s\n' "${FAKE_COUNT:-3}" ;;
esac
exit 0"#,
    );
    for (count, expected_code, expected_status) in [("3", 0, "passed"), ("2", 1, "failed")] {
        let run = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
            .arg("--docker")
            .arg(&fake)
            .args(["demo", "--json"])
            .env("FAKE_COUNT", count)
            .output()
            .unwrap();
        assert_eq!(run.status.code(), Some(expected_code));
        let report: Value =
            serde_json::from_slice(&run.stdout).expect("stdout must be exactly one JSON value");
        assert_eq!(report["status"], expected_status);
        assert!(String::from_utf8_lossy(&run.stderr).is_empty());
    }
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["run", "--config", "does-not-exist.toml", "--json"])
        .assert()
        .code(2)
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains("could not read"));
}

// @claim:local-io-boundary
#[cfg(unix)]
#[test]
fn drill_keeps_inputs_unchanged_and_outputs_in_configured_paths() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let untouched = tempfile::tempdir().unwrap();
    let canary = "PRIVATE-CANARY-7f0c";
    fs::write(untouched.path().join("not-configured.txt"), canary).unwrap();
    let backup = dir.path().join("backup.sql");
    fs::write(&backup, "SELECT 1;\n").unwrap();
    let before = fs::read(&backup).unwrap();
    let secret = dir.path().join("secret.env");
    fs::write(&secret, "POSTGRES_PASSWORD=local-only-secret\n").unwrap();
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(
        dir.path().join("drill.toml"),
        r#"version=1
[drill]
name="privacy-test"
network="restore-drill-privacy-test"
timeout_seconds=20
report_dir="configured-reports"
signing_key="configured-signing.key"
[postgres]
image="postgres:16-alpine"
container="restore-drill-privacy-db"
database="app"
user="restore_drill"
credential_file="secret.env"
[source]
kind="dump"
path="backup.sql"
format="plain"
[[assertions.sql]]
name="one"
query="SELECT 1"
expect="1"
"#,
    )
    .unwrap();
    let log = dir.path().join("docker.log");
    let fake = make_fake_docker(
        &dir,
        r#"printf '%s\n' "$*" >> "$FAKE_DOCKER_LOG"
case " $* " in
  *" image inspect "*) printf 'sha256:sample-image\n' ;;
  *" exec restore-drill-privacy-db psql "*" -c "*) printf '1\n' ;;
esac
exit 0"#,
    );
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--docker")
        .arg(&fake)
        .args(["run", "--config"])
        .arg(dir.path().join("drill.toml"))
        .arg("--json")
        .env("FAKE_DOCKER_LOG", &log)
        .assert()
        .success();
    assert_eq!(fs::read(&backup).unwrap(), before);
    assert_eq!(
        fs::read_to_string(untouched.path().join("not-configured.txt")).unwrap(),
        canary
    );
    assert!(dir.path().join("configured-signing.key").is_file());
    assert!(dir.path().join("configured-signing.key.pub").is_file());
    assert_eq!(
        fs::read_dir(dir.path().join("configured-reports"))
            .unwrap()
            .count(),
        1
    );
    let log_text = fs::read_to_string(log).unwrap();
    assert!(!log_text.contains(canary));
    assert!(!log_text.contains("local-only-secret"));
}

#[cfg(unix)]
#[test]
fn run_completes_with_a_docker_compatible_engine_and_verifies_report() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("backup.sql"),
        "CREATE TABLE proof(id int);\n",
    )
    .unwrap();
    let secret = dir.path().join("secret.env");
    fs::write(&secret, "POSTGRES_PASSWORD=test-only\n").unwrap();
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o600)).unwrap();
    fs::write(
        dir.path().join("drill.toml"),
        r#"version = 1
[drill]
name = "test-drill"
network = "restore-drill-test"
timeout_seconds = 20
report_dir = "reports"
signing_key = "signing.key"
[postgres]
image = "postgres:16-alpine"
container = "restore-drill-db"
database = "app"
user = "restore_drill"
credential_file = "secret.env"
[source]
kind = "dump"
path = "backup.sql"
format = "plain"
[[assertions.sql]]
name = "database responds"
query = "SELECT 1"
expect = "1"
"#,
    )
    .unwrap();
    let fake = dir.path().join("docker");
    let docker_log = dir.path().join("docker.log");
    fs::write(
        &fake,
        r#"#!/bin/sh
printf '%s\n' "$*" >> "$FAKE_DOCKER_LOG"
case " $* " in
  *" image inspect "*) printf 'sha256:test-image-id\n' ;;
  *" exec restore-drill-db psql "*" -f "*)
    if [ "${FAKE_FAIL_RESTORE:-}" = "1" ]; then printf 'broken archive\n' >&2; exit 1; fi ;;
  *" exec restore-drill-db psql "*" -c "*) printf '1\n' ;;
  *) printf 'ok\n' ;;
esac
exit 0
"#,
    )
    .unwrap();
    let mut permissions = fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake, permissions).unwrap();

    let output = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--docker")
        .arg(&fake)
        .args(["run", "--config"])
        .arg(dir.path().join("drill.toml"))
        .arg("--json")
        .env("FAKE_DOCKER_LOG", &docker_log)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["status"], "passed");
    assert_eq!(report["assertions"][0]["observed"], "1");
    assert_eq!(
        report["images"]["postgres:16-alpine"],
        "sha256:test-image-id"
    );

    let report_path = fs::read_dir(dir.path().join("reports"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .args(["verify"])
        .arg(report_path)
        .arg("--public-key")
        .arg(dir.path().join("signing.key.pub"))
        .assert()
        .success()
        .stdout(predicate::str::contains("signature valid"));

    let commands = fs::read_to_string(&docker_log).unwrap();
    assert!(commands.contains("network create --internal"));
    assert!(commands.contains("--env-file"));
    assert!(commands.contains("rm -f restore-drill-db"));
    assert!(commands.contains("volume rm -f"));
    assert!(commands.contains("network rm restore-drill-test"));

    let broken = Command::new(assert_cmd::cargo::cargo_bin!("restore-drill"))
        .arg("--docker")
        .arg(&fake)
        .args(["run", "--config"])
        .arg(dir.path().join("drill.toml"))
        .arg("--json")
        .env("FAKE_DOCKER_LOG", &docker_log)
        .env("FAKE_FAIL_RESTORE", "1")
        .output()
        .unwrap();
    assert_eq!(broken.status.code(), Some(1));
    let failed: Value = serde_json::from_slice(&broken.stdout).unwrap();
    assert_eq!(failed["status"], "failed");
    assert!(failed["error"].as_str().unwrap().contains("broken archive"));
}
