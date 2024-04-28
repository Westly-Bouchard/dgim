use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool, 

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Log an exercise to dgim's internal database
    Log(LogArgs),
}

#[derive(Args)]
pub struct LogArgs {
    exercise_name: Option<String>,
    date: Option<String>,
    num_sets: Option<u8>,
}
