
use chrono::prelude::*;
use rusqlite::params;
use std::process::exit;

pub struct Database {
    db: rusqlite::Connection,
}

impl Database {
    pub fn open() -> Database {
       // Get data path for users platform
       let mut path = dirs::data_dir().expect("Couldn't resolve data path!");

       path.push("dgim");
       path.push("exercises.record");

       let Ok(db) = rusqlite::Connection::open(path) else {
           eprintln!("Encountered error while opening database! Aborting...");
           exit(1);
       };

       match db.execute("
       CREATE TABLE IF NOT EXISTS exercises (
       name TEXT NOT NULL,
       date TEXT NOT NULL,
       sets INTEGER,
       weights TEXT NOT NULL,
       reps TEXT NOT NULL,
       volume INTEGER
       );
       ",  ()) {
           Ok(_) => Database{ db },
           Err(e) => {
               eprintln!("Failed to initialize database. {:?} Aborting...", e);
               exit(1);
           }
       }
    }

    pub fn write(&self, record: Record) {
        let query = "
        INSERT INTO exercises (name, date, sets, weights, reps, volume)
        VALUES(?1, ?2, ?3, ?4, ?5, ?6);
        ";

        let res = self.db.execute(query, params![
                                  record.name,
                                  record.date.to_string(),
                                  record.weights.len(),
                                  record.reps_as_blob(), 
                                  record.weights_as_blob(),
                                  record.get_volume()
        ]);

        match res {
            Ok(n) => {
                if n != 1 {
                    panic!("Number of rows changed in db should be 1, but instead is {}", n);
                }
            }
            Err(e) => {
                eprintln!("Error writing data to local db. {:?} Aborting...", e);
                exit(1);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Record {
    name: String,
    date: DateTime<Local>,
    weights: Vec<f32>,
    reps: Vec<u8>,
}

impl Record {
    pub fn new(name: String, date: DateTime<Local>) -> Self {
        Record {
            name,
            date,
            ..Default::default()
        }
    }

    pub fn add_set(&mut self, weight: f32, reps: u8) {
        self.weights.push(weight);
        self.reps.push(reps);
    }

    fn get_volume(&self) -> f32 {
        self.weights.iter().zip(&self.reps)
            .map(|(&a, &b)| a * b as f32)
            .sum()
    }

    fn reps_as_blob(&self) -> String {
        let mut ret: String = Default::default();

        for count in self.reps.iter() {
            ret.push_str(&count.to_string());
        }

        ret
    }

    fn weights_as_blob(&self) -> String {
        let mut ret: String = Default::default();

        for count in self.weights.iter() {
            ret.push_str(&count.to_string());
        }

        ret
    }

}
