use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    word_to_find: String,
    filename: String
}

fn run(config : Config) -> Result<(), Box<dyn Error>> {
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

impl Config {
    fn new(args: &[String] ) -> Result<Config, &str> {
        if args.len() < 3 {
           return Err("[ERROR] : Not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        return Ok(Config { word_to_find: query.to_string(), filename: filename.to_string()});
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let arg_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for [ {} ] in [ {} ]", arg_config.word_to_find, arg_config.filename);

    if let Err(res) = run(arg_config) {
        println!("{}",res);
        process::exit(1);
    }

}


