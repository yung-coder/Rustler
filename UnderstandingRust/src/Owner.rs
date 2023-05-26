fn main() {
    // Ownership rules
    // Each value  in rust has a variable thats's called its owner
    // There can only be one owner at a time
    // When owner gets out od scope , the value  will be dropped

    let x = 23;
    let y = x;
    println!("Copy in case of int {}", y);

    let s1 = String::from("Move");
    let s2 = s1.clone();

    println!("Its Moved {}", s1);

    // refrence

    let s3 = String::from("Utopia");

    let r1 = s3;
    let r2 = s3;

    println!("{}", r1);

    // Rules for refrences
    // At any given time you can have either one mutable refrences or any number of imutabe
    // refrences
    //
    // Refrences must always be valid
}
