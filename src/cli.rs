use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool, 

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Log an exercise to dgim's internal database
    Log(LogArgs),
}

#[derive(Args, Debug)]
pub struct LogArgs {
    /// Name of the exercise
    #[arg(short, long)]
    name: String,

    /// mm/dd/yyyy
    /// Date the exercise was done
    /// If not specified, the current date will be used
    #[arg(short, long)]
    date: Option<String>,

    /// The Set Data: lb,reps
    sets: Vec<String>,


}
