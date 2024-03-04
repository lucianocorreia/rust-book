#[allow(unused)]
#[allow(unused_variables)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(&coin);
    println!("value: {}", value);

    // if let
    if let Coin::Penny = coin {
        println!("Lucky Penny!")
    } else {
        println!("Not a Penny")
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let four = IpAddrKind::V4(String::from("127.0.0.1"));
//     let six = IpAddrKind::V6(String::from("::1"));

//     route(four);
//     route(six);
// }

// fn route(ip_kind: IpAddrKind) {
//     println!("ip_kind: {:?}", ip_kind)
// }
