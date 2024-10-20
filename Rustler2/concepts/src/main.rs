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
