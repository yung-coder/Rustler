fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);

    let mut v2 = Vec::new();
    v2.extend(&v1);

    assert_eq!(v1, v2);

    println!("Done");

    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];

    let slice2 = &v[0..v.len()];

    assert_eq!(slice1, slice2);

    let vec_ref = &mut v;
    (*vec_ref).push(4);
    let slice3 = &v[0..];

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Done too");
}
