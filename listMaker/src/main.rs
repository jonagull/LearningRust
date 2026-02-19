mod cli;
mod storage;
mod task;

use anyhow::{bail, Result};
use clap::Parser;
use cli::{Cli, Command};
use storage::{load, save};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut tasks = load()?;

    println!("{:#?}", tasks);
    println!("{:#?}", cli.cmd);

    match cli.cmd {
        Command::Add { title } => {
            tasks.push(task::Task::new(title));
            save(&tasks)?;
            println!("Added. Total tasks: {}", tasks.len());
        }

        Command::Ls => {
            if tasks.is_empty() {
                println!("No tasks. Add one with: list add \"...\"");
                return Ok(());
            }

            for (i, t) in tasks.iter().enumerate() {
                let status = if t.done { "[x]" } else { "[ ]" };
                println!("{:>3}. {} {}", i + 1, status, t.title);
            }
        }

        Command::Done { index } => {
            let i = to_zero_based(index, tasks.len())?;
            tasks[i].done = true;
            save(&tasks)?;
            println!("Marked task {} as done.", index);
        }

        Command::Rm { index } => {
            let i = to_zero_based(index, tasks.len())?;
            let removed = tasks.remove(i);
            save(&tasks)?;
            println!("Removed: {}", removed.title);
        }

        Command::Clear => {
            tasks.clear();
            save(&tasks)?;
            println!("Cleared all tasks.");
        }
    }

    Ok(())
}

fn to_zero_based(index_1_based: usize, len: usize) -> Result<usize> {
    if index_1_based == 0 {
        bail!("Index starts at 1.");
    }
    let i = index_1_based - 1;
    if i >= len {
        bail!("No task {} (you have {}).", index_1_based, len);
    }
    Ok(i)
}
