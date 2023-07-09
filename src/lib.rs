use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { //dyn -> dynamic
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}