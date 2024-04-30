#![allow(dead_code)]
use clap::Parser;
use chrono::prelude::*;
use dateparser::parse;
use std::process::exit;

mod config;
mod cli;
mod database;

fn main() {
    // Load config on program start
    let config = config::get_config();

    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Log(args) => {
            println!("Found log command");
            
            // Validate arguments passed

            // Is this exercise a valid exercise?
            if !config.exercises.contains(&args.name) {
                eprintln!("Error: Exercise \"{}\" not found in config!", args.name);
                eprintln!("If this is a new exercise, be sure to add it to your config before logging it");
                exit(1);
            }

            // Handle date
            let mut date = Local::now();

            if let Some(custom_date) = &args.date {
                date = match parse(&custom_date) {
                    Ok(d) => { d.into() }
                    Err(e) => {
                        eprintln!("Error parsing date string: {}, aborting", custom_date);
                        eprintln!("{:?}", e);
                        exit(1);
                    }
                } 
            }


            println!("Found exercise: {}", args.name);
            println!("Using date: {}", date);
        }
    }

}
