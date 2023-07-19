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

    // slice

    let arr = [1, 2, 3, 4, 5];

    let slice = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Final done");

    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Done tuple");
}
