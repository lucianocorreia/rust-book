// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let f = File::open("hello.txt");

//         let mut f = match f {
//             Ok(file) => file,
//             Err(e) => return Err(e),
//         };

//         let mut s = String::new();

//         match f.read_to_string(&mut s) {
//             Ok(_) => Ok(s),
//             Err(e) => Err(e),
//         }
//     }
// }

// #![allow(unused)]
// fn main() {
//     use std::fs::File;
//     use std::io;
//     use std::io::Read;

//     fn read_username_from_file() -> Result<String, io::Error> {
//         let mut f = File::open("hello.txt")?;
//         let mut s = String::new();
//         f.read_to_string(&mut s)?;
//         Ok(s)
//     }
// }

#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut s = String::new();

        File::open("hello.txt")?.read_to_string(&mut s)?;

        Ok(s)
    }
}
