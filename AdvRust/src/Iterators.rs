fn main() {
    let arr = [0; 10];

    for i in arr {
        println!("{}", arr[i]);
    }

    let v1 = vec![1, 2].into_iter();

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}
