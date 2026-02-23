pub struct App {
    pub docker_lines: Vec<String>,
    pub pm2_lines: Vec<String>,
    pub status: String,
}

use crate::services::docker;
use crate::services::pm2;

impl App {
    pub fn new() -> Self {
        Self {
            docker_lines: vec!["(press r to refresh)".to_string()],
            pm2_lines: vec!["(press r to refresh)".to_string()],
            status: "q: quit | r: refresh".to_string(),
        }
    }

    pub fn refresh(&mut self) {
        match docker::list_containers() {
            Ok(containers) => {
                self.docker_lines = containers.into_iter().map(|c| c.to_string()).collect();
            }
            Err(e) => {
                self.docker_lines = vec![format!("Error: {}", e)];
            }
        }
        match pm2::list_processes() {
            Ok(processes) => {
                self.pm2_lines = processes.into_iter().map(|p| p.to_string()).collect();
            }
            Err(e) => {
                self.pm2_lines = vec![format!("Error: {}", e)];
            }
        }
        self.status = format!(
            "docker: {} | pm2: {} | q: quit | r: refresh",
            self.docker_lines.len(),
            self.pm2_lines.len()
        );
    }
}
