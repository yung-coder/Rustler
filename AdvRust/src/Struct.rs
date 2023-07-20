struct Person {
    name: String,
    age: u8,
}

fn build_person(name: String, age: u8) -> Person {
    Person { age, name }
}

enum Number {
    Zero = 0,
    One = 1,
    Two = 2,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg1 = Message::Move { x: 1, y: 2 };
    let msg2 = Message::Write(String::from("Yo we up"));

    println!("ALL good");
}
