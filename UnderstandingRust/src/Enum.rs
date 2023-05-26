#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Option enum

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let localhost = IpAddrKind::V4(String::from("127.0.0.1"));
}

fn route(ip_kind: IpAddrKind) {}
