#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 40,
        height: 50,
    };

    println!("Rect {:#?}", rect);

    println!("hold the value  {}", rect.can_hold(&rect1));

    println!("can hold {}", rect.can_hold(&rect2));

    println!("The are is {}", rect.area());
}
