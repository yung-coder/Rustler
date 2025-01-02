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