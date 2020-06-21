#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut s = String::from("hello");

    change_it(&mut s);

    println!("{}", s);
    println!("{}", s);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: user1.username.clone(),
        ..user1
    };

    println!("{:#?}", user1);
    println!("{:#?}", user2);
}

fn change_it(some_string: &mut String) {
    some_string.push_str(", world");
}