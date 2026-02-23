use serde::Deserialize;
use std::fmt;
use std::io;
use std::process::Command;

#[derive(Debug, Deserialize)]
struct RawPm2Process {
    name: String,
    pm_id: u32,
    pm2_env: Pm2Env,
    monit: Monit,
}

#[derive(Debug, Deserialize)]
struct Pm2Env {
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct Pm2Process {
    id: u32,
    name: String,
    status: String,
    cpu: f64,
    memory: u64,
}

#[derive(Debug, Deserialize)]
struct Monit {
    memory: u64,
    cpu: f64,
}

impl fmt::Display for Pm2Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {} | {} | {}",
            self.id, self.name, self.status, self.cpu, self.memory
        )
    }
}

pub fn list_processes() -> Result<Vec<Pm2Process>, io::Error> {
    let output = Command::new("pm2").arg("jlist").output()?;

    if !output.status.success() {
        return Err(io::Error::new(io::ErrorKind::Other, "pm2 jlist failed"));
    }

    let processes: Vec<RawPm2Process> = serde_json::from_slice(&output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(processes
        .into_iter()
        .map(|p| Pm2Process {
            id: p.pm_id,
            name: p.name,
            status: p.pm2_env.status,
            cpu: p.monit.cpu,
            memory: p.monit.memory,
        })
        .collect())
}
