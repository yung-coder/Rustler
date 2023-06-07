use std::ops::Deref;
use std::rc::Rc;

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

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b= {}", b);

    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    // simple as basic pointer
    assert_eq!(5, x);
    assert_eq!(5, *(z.deref()));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
