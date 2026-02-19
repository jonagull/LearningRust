use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "list", version, about = "A tiny CLI todo list")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add { title: String },
    Ls,
    Done { index: usize },
    Rm { index: usize },
    Clear
}
