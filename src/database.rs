
use chrono::prelude::*;

pub struct Connection {
    db: sqlite::Connection,
}

impl Connection {
    pub fn open(file: &str) -> () {
       // Get data path for users platform
       let mut path = dirs::data_dir().expect("Couldn't resolve data path!");

       path.push("dgim");
       path.push("exercises");

//       match sqlite::open(path) {
//           Ok(con) => {
//                // TODO
//           }

 //          Err(err) => {
 //              eprintln!("{:?}", err);
 //              panic!("AHHH");
 //          }
 //      } 
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
}
