use std::fmt;
use std::fmt::Display;

struct Test {
    x: i32,
    y: i32,
}
impl Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Copy for Test {}
impl Clone for Test {
    fn clone(&self) -> Test {
        *self
    }
}

struct User {
    name: String,
    email: String,
}
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.email)
    }
}

fn main() {
    {
        let x = 5;
        let y = x;
        println!("{}, {}", x, y)
    }
    {
        let s1 = String::from("abc");
        let s2 = s1;
        println!("{}", s2);
        not_take_ownership(&s2);
        println!("{}", s2);
        take_ownership(s2);
    }

    {
        let t1 = Test { x: 1, y: 2 };
        let mut t2 = t1;
        t2.x = 4;
        println!("{:#}, {:#}", t1, t2);
    }

    {
        let name = String::from("name");
        let email = String::from("email");
        let mut u1: User = User { name, email };
        let nn = String::from("name");
        let mut u2: User = User { name: nn, ..u1 };
        u1.email = String::from("u1");
        println!("{}", u1);

        println!("{}", u2);
        u2.email = String::from("u2");
        println!("{}", u2);
    }

    println!("Hello, world!");
}

fn take_ownership(str: String) {
    println!("{}", str)
}
fn not_take_ownership(str: &String) {
    println!("{}", str)
}
