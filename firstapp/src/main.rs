use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let filename = &args[2];

    println!("Args {:?}", args);
    println!("Searching for  {}", query);
    println!("Filename is {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading a file");

    println!("With text:\n{}", contents);
}
