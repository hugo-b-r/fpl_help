use std::{env, process};
use geocoding::{Openstreetmap, Forward, Point};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use dms_coordinates::DMS;

fn main() {
    let osm = Openstreetmap::new();
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem found when getting arguments: {}", err);
        process::exit(1);
    });
    let file = FPL::new(config).unwrap_or_else(|err| {
        eprintln!("Problem found when loadign file: {}", err);
        process::exit(1);
    });

    for address in file.addresses {
        let res: Vec<Point<f64>> = osm.forward(&address).unwrap();
        
        let longitude_decimal = res[0].x();
        let latitude_decimal = res[0].y();
        
        let longitude_dms = DMS::from_decimal_degrees(longitude_decimal, false);
        let latitude_dms = DMS::from_decimal_degrees(latitude_decimal, false);


        let latitude_deg_str: String;
        let latitude_min_str: String;
        let longitude_deg_str: String;
        let longitude_min_str: String;

        if longitude_dms.degrees < 10 {
            longitude_deg_str = format!("00{}", longitude_dms.degrees.to_string());
        } else if longitude_dms.degrees < 100 { //longitude is max 180 compared to 90 for latitude
            longitude_deg_str = format!("0{}", longitude_dms.degrees);
        } else {
            longitude_deg_str = longitude_dms.degrees.to_string();
        }
        if longitude_dms.minutes < 10 {
            longitude_min_str = format!("0{}", longitude_dms.minutes.to_string());
        } else {
            longitude_min_str = longitude_dms.minutes.to_string();
        }
        if latitude_dms.degrees < 10 {
            latitude_deg_str = format!("0{}", latitude_dms.degrees);
        } else {
            latitude_deg_str = latitude_dms.degrees.to_string();
        }
        if latitude_dms.minutes < 10 {
            latitude_min_str = format!("0{}", latitude_dms.minutes.to_string());
        } else {
            latitude_min_str = latitude_dms.minutes.to_string();
        }

        println!(
            "{}{}N{}{}E",
            latitude_deg_str,
            latitude_min_str,
            longitude_deg_str,
            longitude_min_str
        );
    }
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
            eprintln!("File not found; {}", err);
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