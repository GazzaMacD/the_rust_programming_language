enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn print_message(&self) {
        if let Self::Write(msg) = self {
            println!("{}", msg);
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}

fn main() {
    let _home = MyIpAddr::V4(127, 0, 0, 1);
    let _loopback = MyIpAddr::V6(String::from("::1"));
    let coin1 = Coin::Dime;
    println!("Value of coin1 is {}", coin1.value_in_cents());
    // testing out if let statement
    let msg1 = Message::Write(String::from("Hi Bob."));
    msg1.print_message();
}
