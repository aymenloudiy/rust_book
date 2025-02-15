struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
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
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
