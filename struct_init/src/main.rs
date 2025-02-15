struct User {
    _active: bool,
    _username: String,
    email: String,
    _sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
fn main() {
    let mut user1 = User {
        _active: true,
        _username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        _sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    let e = String::from("email");
    let u = String::from("username");

    let _user3 = build_user(e, u);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _subject = AlwaysEqual;

    println!(
        "{},{},{},{},{},{}",
        _origin.0, _black.0, _origin.1, _black.1, _origin.2, _black.2
    );
}

fn build_user(email: String, _username: String) -> User {
    User {
        _active: true,
        _username,
        email,
        _sign_in_count: 1,
    }
}
