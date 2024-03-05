use std::collections::HashMap;

#[allow(unused)]

// Hash Maps
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{field_name}, {field_value}");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

// strings & alisces
// fn main() {
//     let mut s1 = String::from("foo");
//     let s2 = "bar";

//     s1.push_str(s2);
//     println!("s2 is {s2}");

//     let hello = "Здравствуйте";

//     let s = &hello[0..4];
//     println!("s is {s}");

//     for c in hello.chars() {
//         println!("{}", c);
//     }

//     for b in hello.bytes() {
//         println!("{}", b);
//     }
// }

// Vectors
// fn main() {
//     let v = [1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("The third element is {third}");
//
//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(v) => println!("The third element is {v}"),
//         None => println!("There is no third element."),
//     }

//     // iterating over the values in a vector
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }

//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//         println!("{}", i);
//     }
// }
