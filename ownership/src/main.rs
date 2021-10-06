fn main() {
    variables();
    variables_clone();
    stack_only_copy();
    ownsership_functions();
    return_values_scopes();
    references();
    mut_references();
}

fn mut_references() {
    let mut s = String::from("hello");
    change(&mut s);

    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn references() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}

fn return_values_scopes() {
    let s1 = giver_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);

    println!("{}, {}", s1, s3);
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}

fn giver_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn ownsership_functions() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    make_copy(x);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn stack_only_copy() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn variables_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn variables() {
    let mut s = String::from("hello");

    s.push_str(", world");

    println!("{}!", s);
}
