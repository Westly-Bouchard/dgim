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

    match cli.command {
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

            if let Some(custom_date) = args.date {
                date = match parse(&custom_date) {
                    Ok(d) => { d.into() }
                    Err(e) => {
                        eprintln!("Error parsing date string: {}, aborting", custom_date);
                        eprintln!("{:?}", e);
                        exit(1);
                    }
                } 
            }

            // Parse the sets completed
            let mut record = database::Record::new(args.name, date);

            args.sets.into_iter().enumerate()
                .for_each(|(i, set)| {
                    let values: Vec<&str> = set.split(',').collect();

                    let Ok(weight) = values[0].parse::<f32>() else {
                        eprintln!("Could not parse weight {0} from set {1}, aborting", values[0], i);
                        exit(1);
                    };

                    let reps = if let Ok(res) = values[1].parse::<u8>() {
                        res
                    } else {
                        eprintln!("Could not parse weight {0} from set {1}, aborting", values[1], i);
                        exit(1);
                    };

                    let Ok(reps) = values[1].parse::<u8>() else {
                        eprintln!("Could not parse weight {0} from set {1}, aborting", values[1], i);
                        exit(1);
                    };

                    record.add_set(weight, reps);
                });

            println!("{:?}", record);

            // By now we have a valid record, so we can write it to the database
        }
    }

}
