fn main() {
    let u1 = User {
        username: String::from("Daniel Fernando"),
        email: String::from("dfcubidesc@pepito.com"),
        sign_in_count: 10,
        active: false,
    };

    println!("Hello {} , your email is {}", u1.username, u1.email);
    let mut u2 = User {
        username: String::from("Daniel Fernando"),
        email: String::from("dfcubidesc@pepito.com"),
        ..u1
    };

    u2.email = String::from("other@mail.com");
    println!("Hello {} , your status is {}", u2.username, u2.active);

    let u3 = build_user(String::from("mail@other.com"), String::from("dafer"));
    println!("Hello {} , logins attempts {}, and status {}", u3.username, u3.sign_in_count, u3.get_status());
}
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User{
    fn get_status(&self) -> bool {
        self.active
    }
}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}