fn main() {
    variables();
    numeric_operations();
    tuples();
    array();
    if_else();
    repetition();
}

fn repetition() {
    // loop {
    //     println!("Again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Number while: {}", number);

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for x in (1..4).rev() {
        println!("{}!", x);
    }
}

fn if_else() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("If Inline: {}", number);
}

fn array() {
    let a = [1, 2, 3, 4, 5];
    println!("Array: {:?}", a);
}

fn tuples() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Tuple: {} {} {}", x, y, z);

    println!("Tuple: {} {} {}", tup.0, tup.1, tup.2);
}

fn numeric_operations() {
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let mult = 4 * 30;
    let sub = 56.7 / 32.2;
    let rest = 23 % 5;

    println!("Values: {} {} {} {} {}", sum, diff, mult, sub, rest);
}

fn variables() {
    // mut variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("Te value of y is: {}", y);

    let spaces = "     ";
    let spaces = spaces.len();
    println!("The spaces len is: {}", spaces);

    let f = 2.8;
    let g: f32 = 3.0;
    println!("Float {} : {}", f, g);

    let t = true;
    println!("Bool: {}", t);
}
