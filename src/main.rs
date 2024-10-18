use std::env;
use std::process;
use minigrep::Config;
use colored::Colorize;

fn main() {

    let args: Vec<String> = env::args().collect();
    let mut case_sensitive: bool = false;


    if args.len() > 3 && args.contains(&String::from("--case_sensitive")) {
        case_sensitive = true;
    }
    else if args.len() > 3 && !args.contains(&String::from("--case_sensitive")) {
        println!("{} Please specify the --case_sensitive flag", "ERROR: ".red().bold());
        process::exit(1);
    }
    let arg_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for [ {} ] in [ {} ].......", arg_config.word_to_find, arg_config.filename);

    if let Err(res) = minigrep::run(arg_config, case_sensitive) {
        println!("{}",res);
        process::exit(1);
    }

}


