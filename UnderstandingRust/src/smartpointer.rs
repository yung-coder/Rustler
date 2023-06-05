use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b= {}", b);

    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    // simple as basic pointer
    assert_eq!(5, x);
    assert_eq!(5, *(z.deref()));
}
