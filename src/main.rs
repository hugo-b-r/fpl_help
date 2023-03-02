use std::env;

fn main() {
    println!("Hello, world!");
}

struct Config {
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 1 {
            return Err("Please give a file name!");
        }

        let file_name = args[1].clone();

        Ok(Config {
            file_name
        })
    }
}