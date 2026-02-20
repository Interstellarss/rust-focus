use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name= "rust-focus")]
#[command(about = "A CLI productivity tool (Todo + Pommodoro)", long_about = None)]
pub struct Cli {
#[command(subcommand)]
pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add{
        title: String
    },
    List,
    Done{
        id:u64,
    },
    Delete{
        id:u64,
    },
}