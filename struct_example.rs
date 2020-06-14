struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = build_user(String::from("email"), String::from("username"));
    user1.email = String::from("arjun.sumal@gmail.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("yetAnother@gmail.com"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
