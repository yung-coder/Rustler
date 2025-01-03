// 1 Memory theory ( NO CODE )  (done)
// Mutablity (done)

// Moving 

fn main(){
    let s1 = String::from("Rusty");
    do_some(s2:s1);
    println!("number {}", s1); // --> not valid s1 is moved to the function
}


fn do_some(s2: string) {
    println!("{}" , s2)
}

// Borrowing 


fn main(){
    let s1 = String::from("Rusty");
    do_some(s2:&s1); // --> borrowing the s1 
    println!("number {}", s1); // --> not valid s1 is moved to the function
}


fn do_some(s2: &string) {
    println!("{}" , s2)
}

// At any given time you can have either one mutable refernce or any number of immutabl references  


// Vectors

let main() {
    let  mut vec = Vec::new();
    let mut vec2 = vec![1,2,3,4]
    vec.push(1);
    vec.push(2);

    println!("{:?}", vec);
}

fn even(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec
}


// hashmaps 

fn main(){
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(k:String::from("chad"), v:22);
    users.insert(k:String::from("me"), v:32);

    let first_user_age = users.get("chad"); //Option<22>

    match first_user_age {
        Some(age) => println!("age is {}", age);
        None => println!("User not found in the db");
    }
}

// Iterators 

// iterating over for loops 

// mutable iterator 

fn main(){
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.iter_mut(); // mutable iterator 

    for val in v1_iter {
        *val = *val + 1
    }

    println!("{:?}" ,v1);
}


// iterators .next 


fn main(){
    let mut v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter_mut()

    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }

    println!("{:?}" ,v1);
}


// IntoIter -- this is by default 

fn main(){
    let mut v1 = vec![1, 2, 3];

    let v1_iter = v1.into_iter(); // takes the owenership  

    for val in v1_iter {
       println!("{}", val);
    }

    println!("{:?}" ,v1); // v1 becomes invalid 
}


// Iterators adapters -- dose not return 

fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum(); // ownership moved 

    println!("Sum is {}" , sum);
}


// Iterators adapters --  return 

fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let v1_iter2 = v1_iter.map(|x| x +1);

    for i in v1_iter2 {
        println!("{}", i);
    }

    println!("{:?}" ,v1);
}

// Slices 

fn main(){
    let mut word = String::from("new");
    let word2 = find_first(&word);
    println!("{}", word);
}


fn find_first(word: String) -> &str {
    let mut index = 0;
    for (_, i) in words.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}

// traits 

trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("The name is {}, and the age is{}", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Saksham");
        age: 21,
    };
    println!("{}", user.summarize());
}

// lifetimes

fn main(){
    let ans;

    let str1 = String::from("aa");
    {
        let srt2 = String::from("sadsadasda");

        ans = longest(&str1, &str2);
    }
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

// threads 

fn main() {
    thread::spaawn(|| {
        let mut c = 0;
        for i in 0..500000000{
            for i in 0..5000000000{
                c = c + 1;
                c = c - 1;
            }
        }
    });


    let mut c =0;
    for i in 0..5000000{
        for i in 0..50000000{
            c = c + 1;
            c = c - 1;
        }
    }
}

// message passing 

fn main(){
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        tx.send(String::from("Check")).unwrap();
    });

    let value = rx.recv().unwrap();
    println!("{}", value);
}

// Rust bootcamp completed only async rust left 