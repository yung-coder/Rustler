fn main() {
    let x = String::from("Yo");
    let y = x.clone(); // --> if we dont use clone it throw error as the owenership is passed
    println!("{} {}", x, y);

    let z = Box::new(5);

    let mut h = Box::new(1);

    *h = 4;

    assert_eq!(*z, 5);

    println!("Checking");

    // borrowing

    let mut s: String = String::from("hello, ");

    let r1: &mut String = &mut s;
    r1.push_str("world");

    let r2: &mut String = &mut s;
    r2.push_str("!");

    // println!("{}", r1);
}
