mod search;

use std::error::Error;
use std::{env, fs};

use search::{search_file, search_file_case_insensitive};
pub struct Config {
    pub regex: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            regex: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = match config.ignore_case {
        true => search_file_case_insensitive(&config.regex, &contents),
        false => search_file(&config.regex, &contents),
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
