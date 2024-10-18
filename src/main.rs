use std::env;
use std::process;
use minigrep::Config;
fn main() {

    let args: Vec<String> = env::args().collect();
    let arg_config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for [ {} ] in [ {} ]", arg_config.word_to_find, arg_config.filename);

    if let Err(res) = minigrep::run(arg_config) {
        println!("{}",res);
        process::exit(1);
    }

}


