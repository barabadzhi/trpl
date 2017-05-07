#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);

    value_in_cents(&coin);

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("Quarters count: {}", count);
}

fn value_in_cents(coin: &Coin) -> i32 {
    match *coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(ref state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
