
use chrono::prelude::*;
use dateparser::parse;
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

    pub fn get_records(&self, name: String, max: u8) -> Vec<Record> {
        let Ok(mut query) = self.db.prepare("SELECT * FROM exercises WHERE name = ?1 LIMIT ?2;") else {
            panic!("If this is pacicing something is very wrong");
        };
        
        let Ok(mut rows) = query.query(params![name, max]) else {
            eprintln!("Enocuntered error while executing query");
            exit(1);
        };

        let mut ret: Vec<Record> = Vec::new();

        while let Some(row) = rows.next().expect("I don't even know what this error would be") {
            let Ok(name) = row.get::<usize, String>(0) else {
                eprintln!("Problem getting name from row");
                exit(1);
            };

            let date_col: String = row.get(1).expect("Problem getting date from row");

            let Ok(date) = parse(&date_col) else {
                eprintln!("Problem parsing date from row");
                exit(1);
            };

            let weights: Vec<f32> = row.get::<usize, String>(3).expect("Problem getting weights string from row")
                .split(',')
                .map(|weight| {
                    println!("{}", weight);
                    match weight.parse::<f32>() {
                        Ok(w) => w,
                        Err(e) => {
                            eprintln!("Error parsing float from weight string. {:?}", e);
                            exit(1);
                        }
                    }
                }).collect();

            let reps: Vec<u8> = row.get::<usize, String>(4).expect("Problem getting reps string from row")
                .split(',')
                .map(|rep| {
                    println!("{}", rep);
                    match rep.parse::<u8>() {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("Error parsing int from reps string. {:?}", e);
                            exit(1);
                        }
                    }
                }).collect();


            ret.push(Record {
                name,
                date: date.into(),
                weights,
                reps
            });
        }

        ret
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

        for (i, count) in self.reps.iter().enumerate() {
            ret.push_str(&count.to_string());
            if i != self.reps.len() - 1 {
                ret.push(',');
            }            
        }

        ret
    }

    fn weights_as_blob(&self) -> String {
        let mut ret: String = Default::default();

        for (i, count) in self.weights.iter().enumerate() {
            ret.push_str(&count.to_string());
            if i != self.reps.len() - 1 {
                ret.push(',');
            }          
        }

        ret
    }

}
