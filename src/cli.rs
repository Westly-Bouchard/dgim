use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub verbose: bool, 

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Log an exercise to dgim's internal database
    Log(LogArgs),

    // List past records of an exercise
    List(ListArgs),

    // Remove a record by its ID
    Remove(RemoveArgs),
}

#[derive(Args, Debug)]
pub struct LogArgs {
    /// Name of the exercise
    pub name: String,

    /// mm/dd/yyyy
    /// Date the exercise was done
    /// If not specified, the current date will be used
    #[arg(short, long)]
    pub date: Option<String>,

    /// The Set Data: lb,reps
    pub sets: Vec<String>,


}

#[derive(Args, Debug)]
pub struct ListArgs {
    /// Name of exercise to list
    pub name: String,

    /// Max number of records to display (defualt is 10)
    #[arg(short, long)]
    pub num: Option<u8>,
}

#[derive(Args, Debug)]
pub struct RemoveArgs {
    // ID of record to remove
    pub id: i32,
}
