use add_one;

fn main() {
    let num: i32 = 10;
    println!("The number {} plus one = {}", num, add_one::add_one(num));
}
