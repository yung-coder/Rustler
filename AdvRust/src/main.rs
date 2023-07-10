fn main() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Done");

    let z;

    [.., z] = [1, 2];
    assert_eq!(z, 2);

    println!("Done this too");
}
