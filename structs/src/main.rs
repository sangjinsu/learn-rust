struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    user2.email = String::from("anotheremail@example.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.2);
    println!("{:?} {:?}", black, origin);

    let subject = AlwaysEqual;
    println!("{:?}", subject);
}


fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}