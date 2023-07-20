fn match_number(n: i32) {
    match n {
        1 => println!("One!"),

        2..=5 => println!("match 2->5"),
    }
}

fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'));
    }

    println!("Success");
}
