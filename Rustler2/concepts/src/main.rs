fn main() {
    // strings are a bit complex in rust 

    let  _ax =String::from("MeYou");
    println!("{}", _ax);


    // print!("{}", _ax.chars().nth(0)) 


    // basic loops
    // 0 to 9 
    for i in 0..10 {
        print!("{}", i);
    }

    /// for iteration of strings 
    
    let sen = String::from("Lets see rust");
    let first = get_first(sen); 
    print!("First word is: {}", first);


    let a = 32;
    let b = 34;
    let sum = do_sum(a , b);
    println!("Sum is {}", sum);

}


// complexity lies underround 

fn get_first(sen: String) -> String {
    let mut ans = String::from("");
    for char  in sen.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return  ans;
}


fn do_sum(a: i32 , b: i32) -> i32 {
    return a + b;
}

// recap pluss final lap

//{} --> dynamically getting vars  

// recap of basics done now specfic concepts and advance concepts 

// Structs 

// struct example 

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// stuct implement 

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

enum Shape {
    Rectangle(f64 , f64),
    Circle(f64),
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    }

    println!("area is {}", rect1.area());
    println!("area is {}", rect1.perimeter());

    let rect = Shape::Rectangle(1.0, 2.0);
    calculate(shape: rect);

    let index = find(String::from("preet"));

    match index {
        Some(value) => println!("index is {}" , value),
        None => println!("a not found"),
    }

    // Result 

    let result = read_to_string(path: "a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while  reading"),
    }
}

fn calculate(shape: Shape){
    // pattern matching syntax
    let area = match shape {
        Shape::Rectangle(a, b) => a *b,
        Shape::Circle(r) => 3.14 * r *r,
    };
    return area;
}

fn find(s: String) -> Option<i32> {
    for(index, char) in s.chars().enumrate() {
        if (char == 'a'){
            return Some(index as i32)
        }
    }

    return None;
}

use std::fs::read_to_string;

