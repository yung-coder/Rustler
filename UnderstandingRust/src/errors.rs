use ::std::net::IpAddr;

fn main() {
    let var = 22;
    /* if var == 22 {
        panic!("Ohhh Daddyy!!");
    } */

    // result enum
    /* enum Result<T, E> {
        Ok(T),
        Err(E),
    } */
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    println!("{}", home);
}
