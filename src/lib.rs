use std::error::Error;
use std::fs;

pub struct Config {
    pub word_to_find: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String] ) -> Result<Config, &str> {
        if args.len() < 3 {
           return Err("[ERROR] : Not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        return Ok(Config { word_to_find: query.to_string(), filename: filename.to_string()});
    }
}

pub fn run(config : Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in contents.split('.') {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains(&config.word_to_find) {
            println!("Found! [ {} ] in the line [ {} ]", &config.word_to_find, line);
            return Ok(());
        }
    }

    println!("Not found [ {} ] in the file [ {} ]", &config.word_to_find, &config.filename);
    return Ok(());
}


