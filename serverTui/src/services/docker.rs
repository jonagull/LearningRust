use serde::Deserialize;
use std::fmt;
use std::io;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct DockerPsRow {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Names")]
    names: String,
    #[serde(rename = "Image")]
    image: String,
    #[serde(rename = "Status")]
    status: String,
}

#[derive(Debug, Clone)]
pub struct DockerContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
}

impl fmt::Display for DockerContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {}",
            self.id, self.name, self.image, self.status
        )
    }
}

pub fn list_containers() -> Result<Vec<DockerContainer>, io::Error> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{json .}}"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "docker ps failed"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut containers = Vec::new();

    for line in stdout.lines() {
        let parsed: DockerPsRow =
            serde_json::from_str(line).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        containers.push(DockerContainer {
            id: parsed.id,
            name: parsed.names,
            image: parsed.image,
            status: parsed.status,
        });
    }

    Ok(containers)
}
