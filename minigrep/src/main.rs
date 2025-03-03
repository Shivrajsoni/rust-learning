use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments : {} ", err);
        process::exit(1);
    });

    println!("Searching for {} ", config.query);
    print!("In File {} ", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application Error : {}", e);
        process::exit(1);
    }
}
