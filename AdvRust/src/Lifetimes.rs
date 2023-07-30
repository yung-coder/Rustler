fn longest<'a>(x: &'a str, y: &'a str) -> &'a &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x;
        }
        // will not compile as  x can not be accessed outside
        println!("r:{}", r);
    }

    let x = "long";
    let y = "longer";

    println!("{}", longest(x, y));
}
