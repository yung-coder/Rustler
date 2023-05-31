use std::env;
use std::process;

use firstapp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Args {:?}", args);
    println!("Searching for  {}", config.query);
    println!("Filename is {}", config.filename);

    if let Err(e) = firstapp::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
