fn main() {
    let x = String::from("Yo");
    let y = x.clone(); // --> if we dont use clone it throw error as the owenership is passed
    println!("{} {}", x, y);

    let z = Box::new(5);

    let mut h = Box::new(1);

    *h = 4;

    assert_eq!(*z, 5);

    println!("Checking");
}
