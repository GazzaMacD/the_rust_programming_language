// Create user struct with 4 fields, username, email, active, sign_in_count

fn main() {
    user_examples::immutable_users();
    user_examples::mutable_users();
    user_examples::struct_update_syntax()
}

mod user_utils {
    pub struct User {
        pub email: String,
        pub username: String,
        pub active: bool,
        pub sign_in_count: u64,
    }
    pub fn print_user(user: &User) {
        println!(
        "\n\nEmail:\t\t{email}\nUsername:\t{username}\nActive:\t\t{active}\nSignin Count:\t{si_count}",
        email = user.email,
        username = user.username,
        active = user.active,
        si_count = user.sign_in_count
        );
    }

    pub fn create_user(email: String, username: String) -> User {
        User {
            email, // note use of shorthand --> this is equivalent to email: email (like es6)
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

mod user_examples {
    use crate::{user_utils, user_utils::User};

    pub fn immutable_users() {
        // create unmutable user
        let user1 = User {
            email: String::from("myemail@email.com"),
            username: String::from("My Name"),
            active: true,
            sign_in_count: 1,
        };
        user_utils::print_user(&user1);

        let email2 = String::from("bob@gmail");
        let username2 = String::from("bob");
        let user2 = user_utils::create_user(email2, username2);
        user_utils::print_user(&user2);
        user_utils::print_user(&user2);
    }

    pub fn mutable_users() {
        // Mutability
        let mut user3 = User {
            email: String::from("john@email.com"),
            username: String::from("john"),
            active: true,
            sign_in_count: 1,
        };
        user_utils::print_user(&user3);
        user3.email = String::from("john.j@gmail.com");
        println!("User email has been updated");
        user_utils::print_user(&user3);
    }

    pub fn struct_update_syntax() {
        // ===== Struct update syntax - ====
        //  user4 will get new values for email and username
        // and then active and sign_in_count will use the user3 values
        // note: in this case user 3 and user 4 can be used again
        // because active and sign_in_count are 'copy'
        let user4 = User {
            email: String::from("john@email.com"),
            username: String::from("john"),
            active: true,
            sign_in_count: 1,
        };

        let user5 = User {
            email: String::from("sarah@email.com"),
            username: String::from("sarah"),
            ..user4
        };
        user_utils::print_user(&user4);
        user_utils::print_user(&user5);
        // note: in this case user 4 cannot be used after this as
        // email and username are 'move' and not 'copy'

        let user5 = User {
            active: false,
            sign_in_count: 2,
            ..user4
        };
        user_utils::print_user(&user5);
        // print_user(&user4); <-- this line would error
    }
}
