use std::{env, process};
use geocoding::{Forward, Opencage, Point};
use geo_types::Coord;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
}

struct Config {
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please give a file name!");
        }

        let file_name = args[1].clone();

        Ok(Config {
            file_name
        })
    }
}

struct FPL {
    file_name: String,
    addresses: Vec<String>,
}

impl FPL {
    fn new(config: Config) -> Result<FPL, &'static str> {
        let file_name = config.file_name.clone();

        let file = File::open(&file_name).unwrap_or_else(|err| {
            eprintln!("File not found");
            process::exit(1);
        });
        let file_reader = BufReader::new(file);

        let mut addresses = Vec::new();

        for line in file_reader.lines() {
            addresses.push(line.unwrap().to_string());
        }
        Ok(FPL {
            file_name,
            addresses,
        })
    }
}