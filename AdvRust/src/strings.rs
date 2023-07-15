fn main() {
    let s1 = String::from("hi, my name");
    let h = &s1[0..1];
    assert_eq!(h, "h");

    let h1 = &s1[3..5];

    // you can concat String and &str

    let w1 = String::from("hello,");
    let w2 = String::from("world!");
    let w3 = w1 + w2.as_str();
    assert_eq!(w3, "hello,world");
    println!("{}", w1);
}
