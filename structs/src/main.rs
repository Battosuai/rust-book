// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit like struct
struct AlwaysEqual;

struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // user1 values has been moved. can no longer use user1
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // println!("Value: {user1}");
    println!("Value: {user3:?}");
    println!("Value: {user3:#?}");
    // dbg! takes ownership
    dbg!(&user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
