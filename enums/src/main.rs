use std::fmt;

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IpAddrKind::V4(ref addr_0, ref addr_1, ref addr_2, ref addr_3) => {
                write!(f, "{}.{}.{}.{}", addr_0, addr_1, addr_2, addr_3)
            }
            IpAddrKind::V6(ref address) => write!(f, "{}", address),
        }
    }
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self, ip_type: &IpAddrKind) {
        match *self {
            Message::Write(ref message) => {
                println!("Writing message {} to {}...", message, ip_type)
            }
            _ => println!("Not a write message activity..."),
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));

    m.call(&home);
    m.call(&loopback);
}
