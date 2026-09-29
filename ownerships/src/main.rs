fn main() {
    let s = String::from("Hello");
    let s = takes_value(s);
    println!("Hello, world- {s}");

    let s1 = String::from("Hello World");
    let len = calculate_length(&s1);
    println!("Length of '{s1}' is {len}");

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 : {s2}");

    let s = String::from("hello world");

    let hello: &str = &s[..5];
    let world = &s[6..];

    println!("hello : {hello}, world : {world}");

    let s4 = first_word(&s);
    println!("s4 : {s4}");
}

fn takes_value(s1: String) -> String {
    println!("Hello {s1}");
    return s1;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
