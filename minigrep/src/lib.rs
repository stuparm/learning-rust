use std::error::Error;
use std::fs;

pub fn run(config: Config)  -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str)  -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}


