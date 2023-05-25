use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Please enter the number");

    let nub: u32 = rand::thread_rng().gen_range(1..10);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Not anle to read it");

        println!("You guessed {}", guess);

        let guess: u32 = guess.trim().parse().expect("Err");

        match guess.cmp(&nub) {
            Ordering::Less => println!("Small"),
            Ordering::Greater => println!("Big"),
            Ordering::Equal => {
                println!("Win");
                break;
            }
        }
    }
}
