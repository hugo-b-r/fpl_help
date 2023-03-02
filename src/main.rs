use std::{env, process};
use fpl_help::{Config, FPL, get_addresses};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem found when getting arguments: {}", err);
        process::exit(1);
    });
    let file = FPL::new(config).unwrap_or_else(|err| {
        eprintln!("Problem found when loadign file: {}", err);
        process::exit(1);
    });

    println!( "{}", get_addresses(file).unwrap() );
}
