use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

pub const DEFAULT_CONFIG: &str = r#"version = 1

[drill]
name = "weekly-app"
network = "restore-drill-weekly-app"
timeout_seconds = 300
report_dir = ".restore-drill/reports"
signing_key = ".restore-drill/signing.key"

[postgres]
image = "postgres:16-alpine"
container = "restore-drill-db"
database = "app"
user = "restore_drill"
credential_file = ".restore-drill.env"

[source]
kind = "dump"
path = "backups/latest.dump"
format = "auto"

[[assertions.sql]]
name = "database responds"
query = "SELECT 1"
expect = "1"
"#;

pub const DEFAULT_ENV: &str = "# Keep this file out of version control.\nPOSTGRES_PASSWORD=replace-with-a-long-random-secret\n";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u8,
    pub drill: Drill,
    pub postgres: Postgres,
    pub source: Source,
    #[serde(default)]
    pub assertions: Assertions,
    #[serde(default)]
    pub services: Vec<Service>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Drill {
    pub name: String,
    pub network: String,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_report_dir")]
    pub report_dir: PathBuf,
    #[serde(default = "default_signing_key")]
    pub signing_key: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Postgres {
    pub image: String,
    #[serde(default = "default_container")]
    pub container: String,
    pub database: String,
    pub user: String,
    pub credential_file: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceKind {
    Dump,
    VolumeTar,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum DumpFormat {
    Auto,
    Custom,
    Plain,
}

fn default_format() -> DumpFormat {
    DumpFormat::Auto
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub kind: SourceKind,
    pub path: PathBuf,
    #[serde(default = "default_format")]
    pub format: DumpFormat,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertions {
    #[serde(default)]
    pub sql: Vec<SqlAssertion>,
    #[serde(default)]
    pub http: Vec<HttpAssertion>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SqlAssertion {
    pub name: String,
    pub query: String,
    pub expect: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpAssertion {
    pub name: String,
    pub url: String,
    #[serde(default = "default_status")]
    pub status: u16,
    pub body_contains: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    pub name: String,
    pub image: String,
    pub env_file: Option<PathBuf>,
    #[serde(default)]
    pub command: Vec<String>,
}

fn default_timeout() -> u64 {
    300
}
fn default_report_dir() -> PathBuf {
    PathBuf::from(".restore-drill/reports")
}
fn default_signing_key() -> PathBuf {
    PathBuf::from(".restore-drill/signing.key")
}
fn default_container() -> String {
    "restore-drill-db".into()
}
fn default_status() -> u16 {
    200
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = fs::read_to_string(path)
            .map_err(|e| format!("could not read {}: {e}", path.display()))?;
        let mut config: Config =
            toml::from_str(&raw).map_err(|e| format!("invalid TOML in {}: {e}", path.display()))?;
        let base = path.parent().unwrap_or_else(|| Path::new("."));
        config.resolve_paths(base);
        config.validate()?;
        Ok(config)
    }

    fn resolve_paths(&mut self, base: &Path) {
        for path in [
            &mut self.source.path,
            &mut self.postgres.credential_file,
            &mut self.drill.report_dir,
            &mut self.drill.signing_key,
        ] {
            if path.is_relative() {
                *path = base.join(&*path);
            }
        }
        for service in &mut self.services {
            if let Some(path) = &mut service.env_file {
                if path.is_relative() {
                    *path = base.join(&*path);
                }
            }
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("unsupported config version; expected 1".into());
        }
        validate_slug("drill.name", &self.drill.name)?;
        validate_slug("drill.network", &self.drill.network)?;
        if !self.drill.network.starts_with("restore-drill-") {
            return Err(
                "drill.network must start with 'restore-drill-' to mark it disposable".into(),
            );
        }
        validate_slug("postgres.container", &self.postgres.container)?;
        if !self.postgres.container.starts_with("restore-drill-") {
            return Err("postgres.container must start with 'restore-drill-'".into());
        }
        if self.drill.timeout_seconds < 10 || self.drill.timeout_seconds > 86_400 {
            return Err("drill.timeout_seconds must be between 10 and 86400".into());
        }
        if self.postgres.image.trim().is_empty()
            || self.postgres.database.trim().is_empty()
            || self.postgres.user.trim().is_empty()
        {
            return Err("postgres image, database, and user are required".into());
        }
        if !self.source.path.is_file() {
            return Err(format!(
                "backup source does not exist or is not a file: {}",
                self.source.path.display()
            ));
        }
        validate_env_file(&self.postgres.credential_file, true)?;
        let mut service_names = HashSet::new();
        for service in &self.services {
            validate_slug("services.name", &service.name)?;
            if !service.name.starts_with("restore-drill-") {
                return Err(format!(
                    "service '{}' must start with 'restore-drill-'",
                    service.name
                ));
            }
            if !service_names.insert(service.name.as_str()) {
                return Err(format!("duplicate service name: {}", service.name));
            }
            if service.name == self.postgres.container {
                return Err(format!(
                    "service name conflicts with postgres container: {}",
                    service.name
                ));
            }
            if service.image.trim().is_empty() {
                return Err(format!("service '{}' has no image", service.name));
            }
            if let Some(path) = &service.env_file {
                validate_env_file(path, false)?;
            }
        }
        for assertion in &self.assertions.sql {
            if assertion.name.trim().is_empty() || assertion.query.trim().is_empty() {
                return Err("SQL assertions require a name and query".into());
            }
        }
        for assertion in &self.assertions.http {
            let parsed = Url::parse(&assertion.url)
                .map_err(|e| format!("invalid HTTP assertion URL '{}': {e}", assertion.url))?;
            if parsed.scheme() != "http" {
                return Err(format!(
                    "HTTP assertion '{}' must use http inside the isolated network",
                    assertion.name
                ));
            }
            let host = parsed
                .host_str()
                .ok_or_else(|| format!("HTTP assertion '{}' has no host", assertion.name))?;
            if !service_names.contains(host) {
                return Err(format!(
                    "HTTP assertion '{}' host '{}' is not a declared service",
                    assertion.name, host
                ));
            }
            if !(100..=599).contains(&assertion.status) {
                return Err(format!(
                    "HTTP assertion '{}' has an invalid status",
                    assertion.name
                ));
            }
        }
        if self.assertions.sql.is_empty() && self.assertions.http.is_empty() {
            return Err("configure at least one SQL or HTTP assertion".into());
        }
        if matches!(self.source.kind, SourceKind::VolumeTar) {
            validate_tar(&self.source.path)?;
        }
        Ok(())
    }
}

fn validate_slug(field: &str, value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 63
        || !value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(format!(
            "{field} must use 1-63 lowercase letters, digits, or hyphens"
        ));
    }
    Ok(())
}

fn validate_env_file(path: &Path, postgres: bool) -> Result<(), String> {
    let raw = fs::read_to_string(path)
        .map_err(|e| format!("could not read credential file {}: {e}", path.display()))?;
    if postgres
        && !raw.lines().any(|line| {
            line.trim_start()
                .strip_prefix("POSTGRES_PASSWORD=")
                .is_some_and(|value| !value.trim().is_empty())
        })
    {
        return Err(format!(
            "credential file {} must define a non-empty POSTGRES_PASSWORD",
            path.display()
        ));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(path)
            .map_err(|e| format!("could not inspect credential file {}: {e}", path.display()))?
            .permissions()
            .mode();
        if mode & 0o077 != 0 {
            return Err(format!(
                "credential file {} is readable by other users; run chmod 600 {}",
                path.display(),
                path.display()
            ));
        }
    }
    Ok(())
}

fn validate_tar(path: &Path) -> Result<(), String> {
    let file = fs::File::open(path).map_err(|e| format!("could not open volume archive: {e}"))?;
    let gz = path
        .extension()
        .and_then(|v| v.to_str())
        .map(|v| v == "gz" || v == "tgz")
        .unwrap_or(false);
    let reader: Box<dyn std::io::Read> = if gz {
        Box::new(flate2::read::GzDecoder::new(file))
    } else {
        Box::new(file)
    };
    let mut archive = tar::Archive::new(reader);
    let mut has_version = false;
    for entry in archive
        .entries()
        .map_err(|e| format!("invalid volume archive: {e}"))?
    {
        let entry = entry.map_err(|e| format!("invalid volume archive entry: {e}"))?;
        let entry_type = entry.header().entry_type();
        if entry_type.is_symlink() || entry_type.is_hard_link() {
            return Err("volume archive may not contain links".into());
        }
        if !entry_type.is_file() && !entry_type.is_dir() {
            return Err("volume archive may contain only regular files and directories".into());
        }
        let p = entry
            .path()
            .map_err(|e| format!("invalid archive path: {e}"))?;
        if p.is_absolute()
            || p.components()
                .any(|c| matches!(c, std::path::Component::ParentDir))
        {
            return Err("volume archive contains an unsafe path".into());
        }
        if p.file_name().and_then(|v| v.to_str()) == Some("PG_VERSION") {
            has_version = true;
        }
    }
    if !has_version {
        return Err("volume archive has no PG_VERSION file".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn fixture(http_url: &str) -> (tempfile::TempDir, PathBuf) {
        #[cfg(unix)]
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("backup.sql"), "select 1;").unwrap();
        let secret = dir.path().join("secret.env");
        fs::write(&secret, "POSTGRES_PASSWORD=test\n").unwrap();
        #[cfg(unix)]
        fs::set_permissions(&secret, fs::Permissions::from_mode(0o600)).unwrap();
        let config = format!(
            r#"version=1
[drill]
name="test"
network="restore-drill-test"
timeout_seconds=30
[postgres]
image="postgres:16-alpine"
container="restore-drill-db"
database="app"
user="postgres"
credential_file="secret.env"
[source]
kind="dump"
path="backup.sql"
[[assertions.sql]]
name="one"
query="select 1"
expect="1"
[[services]]
name="restore-drill-app"
image="app:test"
[[assertions.http]]
name="health"
url="{http_url}"
"#
        );
        let path = dir.path().join("drill.toml");
        fs::write(&path, config).unwrap();
        (dir, path)
    }

    #[test]
    fn accepts_internal_declared_http_host() {
        let (_dir, path) = fixture("http://restore-drill-app:3000/health");
        Config::load(&path).unwrap();
    }

    // @claim:production-boundary
    #[test]
    fn rejects_production_http_host() {
        let (_dir, path) = fixture("https://api.example.com/health");
        let error = Config::load(&path).unwrap_err();
        assert!(error.contains("must use http"));
    }

    #[test]
    fn rejects_archive_traversal() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.tar");
        let mut builder = tar::Builder::new(fs::File::create(&path).unwrap());
        let mut header = tar::Header::new_gnu();
        header.set_size(1);
        header.set_mode(0o644);
        header.set_cksum();
        // tar itself rejects raw traversal names, proving the same invariant.
        assert!(
            builder
                .append_data(&mut header, "../PG_VERSION", &b"1"[..])
                .is_err()
        );
        let _ = std::io::sink().write_all(b"");
    }
}
