use std::fmt;
use std::fmt::Display;
#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

impl User {
    fn set_email(&mut self, str: String) {
        self.email = str;
    }
}

impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.email)
    }
}

fn main() {
    let mut u1 = User {
        name: String::from("user"),
        email: String::from("email"),
    };

    u1.set_email(String::from("new email"));
    println!("{:#?}", u1);
    println!("{:?}", u1);
    println!("{}", u1);
    println!("Hello, world!");
}
