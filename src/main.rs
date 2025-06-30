use std::fmt::Debug;

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    gender: i8, // 0: female, 1: male, 2: other
    age: i16,
    married: bool,
}

impl User {
    // associated function
    fn create_new(username: String, email: String, gender: i8, age: i16, married: bool) -> Self {
        Self {
            username: username,
            email: email,
            gender: gender,
            age: age,
            married: married,
        }
    }

    // method
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

fn main() {
    let user = User {
        username: String::from("DzungMinh"),
        email: String::from("dzungngminh@gmai.com"),
        gender: 1,
        age: 23,
        married: false,
    };

    let user2 = User::create_new(
        String::from("DzungMinh2"),
        String::from("dzungngminh@gmai.com"),
        1,
        15,
        false,
    );

    let is_user1_adult = user.is_adult();
    let is_user2_adult = user2.is_adult();

    println!("User: {user:?}");
    println!("User2: {user2:?}");

    println!(
        "Is user1 adult: {}",
        if is_user1_adult { "Yes" } else { "No" }
    );
    println!(
        "Is user2 adult: {}",
        if is_user2_adult { "Yes" } else { "No" }
    );
}
