use serde::{Deserialize, Serialize};
use std::process::Command;

const MIRROR_REGISTRY: &str = "quay.io/image-mirror";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub host: String,
    pub path: String,
    pub tags: Vec<Tag>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub name: String,
    pub digest: Option<String>,
    pub is_synced: bool,
}

impl Tag {
    pub fn sync(&mut self, repository: &str) -> Result<&Self, Box<dyn std::error::Error>> {
        let output = Command::new("skopeo")
            .arg("sync")
            .arg("--all")
            .arg("--preserve-digests")
            .arg("--keep-going")
            .args(["--retry-times", "3"])
            .arg("--scoped")
            .args(["--src", "docker"])
            .args(["--src", "docker"])
            .args(["--dest", "docker"])
            .args(["--authfile", "auth.json"])
            .arg(&format!("{}:{}", repository, &self.name))
            .arg(MIRROR_REGISTRY)
            .output()?;
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        if !output.status.success() {
            let output = Command::new("skopeo")
                .arg("sync")
                .arg("--all")
                .args(["--retry-times", "3"])
                .arg("--scoped")
                .args(["--src", "docker"])
                .args(["--src", "docker"])
                .args(["--dest", "docker"])
                .args(["--authfile", "auth.json"])
                .arg(&format!("{}:{}", repository, &self.name))
                .arg(MIRROR_REGISTRY)
                .output()?;
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            if !output.status.success() {
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                return Err("failed to sync".into());
            }
        }
        self.is_synced = true;
        Ok(self)
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.name == other.name && self.digest == other.digest
    }
}
impl Eq for Tag {}
