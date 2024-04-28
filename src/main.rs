#![allow(dead_code)]

mod config;

fn main() {
    let config = config::get_config();

    println!("{:?}", config);
}
