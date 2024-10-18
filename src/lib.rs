use std::error::Error;
use std::fs;
use std::process;
use colored::Colorize;
pub struct Config {
    pub word_to_find: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String] ) -> Result<Config, &str> {
        if args.len() < 3 {
           return Err("Not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        return Ok(Config { word_to_find: query.to_string(), filename: filename.to_string()});
    }
}

fn search(word_to_find: &str, contents: &str, case_sensitive: bool) -> Result<(), Box<dyn Error>> {
    let mut result = Vec::new();
    for line in contents.split('.'){
        let line = line.trim();
        let (search_line, search_word) = if case_sensitive {
            (line.to_string(), word_to_find.to_string()) 
        } else {
            (line.to_lowercase(), word_to_find.to_lowercase()) 
        };
        if !line.is_empty() && search_line.contains(&search_word) {
            result.push(line);
            println!("{} [ {} ] in the line [ {} ]","Found!".green().bold(), word_to_find.yellow().bold(), line.white().dimmed());
            return Ok(());
        }
    }
    println!("{} [ {} ] in the the file","Not Found!".red().bold(), word_to_find.yellow().bold());
    return Ok(());
}

pub fn run(config : Config, case_sensitive: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    if let Err(res) = search(&config.word_to_find, &contents, case_sensitive) {
        println!("{}", res);
        process::exit(1);
    }
    return Ok(());
}


