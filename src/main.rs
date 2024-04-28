#![allow(dead_code)]
use clap::Parser;

mod config;
mod cli;

fn main() {
    // Load config on program start
    let config = config::get_config();

    let cli = cli::Cli::parse();

}
