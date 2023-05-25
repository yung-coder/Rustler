use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    println!("You name");
    let mut name: String = String::new();
    let greetings: &str = "Nice";
    io::stdin().read_line(&mut name);
    println!("Hello {}! {}", name.trim_end(), greetings);

    // data types

    println!("1 + 2 = {}", 2 + 2);

    let age: i32 = 18;
    println!("Age {}", age);
}
