use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub name: String,
    pub kind: String,
    pub passed: bool,
    pub expected: String,
    pub observed: String,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactEvidence {
    pub file_name: String,
    pub kind: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub schema: String,
    pub drill: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub recovery_time_ms: u64,
    pub status: String,
    pub artifact: ArtifactEvidence,
    pub images: BTreeMap<String, String>,
    pub assertions: Vec<Evidence>,
    pub error: Option<String>,
    pub public_key: String,
    pub signature: String,
}

impl Report {
    fn signing_bytes(&self) -> Result<Vec<u8>, String> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        serde_json::to_vec(&unsigned).map_err(|e| format!("could not serialize report: {e}"))
    }

    pub fn sign_and_write(
        &mut self,
        key_path: &Path,
        report_dir: &Path,
    ) -> Result<PathBuf, String> {
        let key = load_or_create_key(key_path)?;
        self.public_key = STANDARD.encode(key.verifying_key().as_bytes());
        self.signature = STANDARD.encode(key.sign(&self.signing_bytes()?).to_bytes());
        fs::create_dir_all(report_dir)
            .map_err(|e| format!("could not create report directory: {e}"))?;
        let stamp = self.finished_at.format("%Y-%m-%dT%H%M%S%.3fZ");
        let path = report_dir.join(format!("{}-{stamp}.json", self.drill));
        let content =
            serde_json::to_vec_pretty(self).map_err(|e| format!("could not encode report: {e}"))?;
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .and_then(|mut file| file.write_all(&content))
            .map_err(|e| {
                format!(
                    "could not create {} without overwriting evidence: {e}",
                    path.display()
                )
            })?;
        Ok(path)
    }

    pub fn verify(&self) -> Result<(), String> {
        let public: [u8; 32] = STANDARD
            .decode(&self.public_key)
            .map_err(|_| "report public key is not valid base64".to_string())?
            .try_into()
            .map_err(|_| "report public key has the wrong length".to_string())?;
        let key = VerifyingKey::from_bytes(&public)
            .map_err(|_| "report public key is invalid".to_string())?;
        let signature = Signature::from_slice(
            &STANDARD
                .decode(&self.signature)
                .map_err(|_| "report signature is not valid base64".to_string())?,
        )
        .map_err(|_| "report signature has the wrong length".to_string())?;
        key.verify(&self.signing_bytes()?, &signature)
            .map_err(|_| "report signature is invalid".to_string())
    }
}

fn load_or_create_key(path: &Path) -> Result<SigningKey, String> {
    if path.exists() {
        let encoded =
            fs::read_to_string(path).map_err(|e| format!("could not read signing key: {e}"))?;
        let bytes: [u8; 32] = STANDARD
            .decode(encoded.trim())
            .map_err(|_| "signing key is not valid base64".to_string())?
            .try_into()
            .map_err(|_| "signing key has the wrong length".to_string())?;
        return Ok(SigningKey::from_bytes(&bytes));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("could not create key directory: {e}"))?;
    }
    let key = SigningKey::generate(&mut OsRng);
    let encoded = STANDARD.encode(key.to_bytes());
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options
        .open(path)
        .map_err(|e| format!("could not create signing key: {e}"))?;
    writeln!(file, "{encoded}").map_err(|e| format!("could not write signing key: {e}"))?;
    let public_path = PathBuf::from(format!("{}.pub", path.display()));
    fs::write(
        public_path,
        format!("{}\n", STANDARD.encode(key.verifying_key().as_bytes())),
    )
    .map_err(|e| format!("could not write public key: {e}"))?;
    Ok(key)
}

pub fn read_and_verify(path: &Path) -> Result<Report, String> {
    let data = fs::read(path).map_err(|e| format!("could not read report: {e}"))?;
    let report: Report =
        serde_json::from_slice(&data).map_err(|e| format!("invalid report JSON: {e}"))?;
    report.verify()?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Report {
        Report {
            schema: "restore-drill/v1".into(),
            drill: "weekly".into(),
            started_at: Utc::now(),
            finished_at: Utc::now(),
            recovery_time_ms: 12,
            status: "passed".into(),
            artifact: ArtifactEvidence {
                file_name: "backup.dump".into(),
                kind: "dump".into(),
                sha256: "abc".into(),
                bytes: 3,
            },
            images: BTreeMap::new(),
            assertions: vec![],
            error: None,
            public_key: String::new(),
            signature: String::new(),
        }
    }

    #[test]
    fn signed_report_detects_tampering() {
        let dir = tempfile::tempdir().unwrap();
        let mut report = sample();
        let path = report
            .sign_and_write(&dir.path().join("key"), dir.path())
            .unwrap();
        read_and_verify(&path).unwrap();
        report.status = "failed".into();
        assert!(report.verify().is_err());
    }
}
