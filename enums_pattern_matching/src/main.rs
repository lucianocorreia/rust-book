fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }

// fn main() {
//     value_in_cents(Coin::Quarter(UsState::Alaska));
// }

// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");

//     let absent_number: Option<i32> = None;
// }

// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }

//     let home = IpAddr::V4(127, 0, 0, 1);

//     let loopback = IpAddr::V6(String::from("::1"));
// }

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {}

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// fn main() {}

// fn main() {
//     enum Message {
//         Quit,
//         Move { x: i32, y: i32 },
//         Write(String),
//         ChangeColor(i32, i32, i32),
//     }

//     impl Message {
//         fn call(&self) {
//             // method body would be defined here
//         }
//     }

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }
