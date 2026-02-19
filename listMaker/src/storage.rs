use crate::task::Task;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn data_path() -> Result<PathBuf> {
    // Store next ot the executable working dir: ./list.json
    Ok(std::env::current_dir()?.join("list.json"))
}

pub fn load () -> Result<Vec<Task>> {
    let path = data_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let bytes = fs::read(&path).with_context(|| format!("Could not read {}", path.display()))?;
    let tasks: Vec<Task>  = serde_json::from_slice(&bytes).with_context(||  "Failed to parse JSON")?;
    Ok(tasks)
}

pub fn save(tasks: &[Task]) -> Result<()> {
    let path = data_path()?;
    let json = serde_json::to_vec_pretty(tasks).with_context(|| "Failed to serialize tasks")?;

    fs::write(&path, json).with_context(|| "Failed to write tasks to JSON");
    Ok(())
}