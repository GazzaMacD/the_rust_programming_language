// Create user struct with 4 fields, username, email, active, sign_in_count
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

fn main() {
    // create unmutable user
    let user1 = User {
        email: String::from("myemail@email.com"),
        username: String::from("My Name"),
        active: true,
        sign_in_count: 1,
    };
    print_user(&user1);

    let email2 = String::from("bob@gmail");
    let username2 = String::from("bob");
    let user2 = create_user(email2, username2);
    print_user(&user2);
    print_user(&user2);
}

fn create_user(email: String, username: String) -> User {
    User {
        email, // note use of shorthand --> this is equivalent to email: email (like es6)
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "\n\nEmail:\t\t{email}\nUsername:\t{username}\nActive:\t\t{active}\nSignin Count:\t{si_count}",
        email = user.email,
        username = user.username,
        active = user.active,
        si_count = user.sign_in_count
    );
}
