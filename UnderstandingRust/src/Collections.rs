fn main() {
    let a = [1, 2, 3];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let v2 = vec![1, 2, 4];
    let third = &v2[2]; // vector are stored on heap
    print!("ele is {}", third);
    println!("Vector {:?}", v2);

    match v2.get(20) {
        Some(third) => println!("Third is there {}", third),
        None => println!("There is no third element"),
    }
}
