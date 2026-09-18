use std::println;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let guess: u32 = "42".parse().expect("Not a number!");

    let tup = (500, 6.4, 1);

    let (a, b, c) = tup;

    println!("The value of b is: {b}");

    // let a = [1, 2, 3, 4, 5];

    // let a: [i32; 5] = [1, 2, 3, 4, 5]; // type and length

    // let a = [3; 5]; // 5 element, value 3

    another_function(9);

    let y1 = {
        let x = 3;
        x + 1
    };

    println!("The value of y1 is: {y1}");

    let y2 = five();

    println!("The value of y2 is: {y2}");

    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let result = check_loop();
    println!("Result: {result}")
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

// Rust has three kinds of loops: loop, while, and for

fn check_loop() -> i32 {
    let mut i = 0;
    loop {
        println!("again!");
        i = i + 1;
        if i == 5 {
            break i * 2;
        }
    }
}

// while example
// while number != 0 {
//     println!("{number}!");

//     number -= 1;
// }

// For example
// for element in a {
//     println!("the value is: {element}");
// }
