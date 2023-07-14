fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Done");

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }

    let t = true;
    if !t {
        println!("Test");
    }

    let v = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    println!("Done!");
}
