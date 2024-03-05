#![allow(unused)]
use std::error::Error;
use std::fs::File;
use std::{
    fs,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn las_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(error) => Err(error),
//     }
// }

// fn main() {
// let greeting_file_result = File::open("hello.txt");

// let greeting_file = match greeting_file_result {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("hello.txt") {
//             Ok(fc) => fc,
//             Err(e) => panic!("Problem creating the file: {:?}", e),
//         },
//         other_error => {
//             panic!("Problem opening the file: {:?}", other_error)
//         }
//     },
// };

// let greeting_file = File::open("hello2.txt").unwrap();

// let greeting_file = File::open("hello2.txt").expect("Failed to open hello2.txt");
// }
