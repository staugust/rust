use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn pr(str: String) -> String {
    let part1 = &str[0..5];
    let part2 = &str[5..9];

    println!("{};{};{}", str, part1, part2);
    str
}

use hello_cargo::test as libtest;
mod test;
fn main() {
    let ch: char = '™';
    println!("{}", ch);

    let x = 5;
    println!("{}", x);
    let x = 6;
    println!("{}", x);

    // as we can see, addr of VALUE in test is not the same.
    test::p(String::from("with mod"));
    libtest::p(String::from("with lib"));

    println!("Hello, world!");
    let mut s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{}.{}.{}", s, part1, part2);
    s = pr(s);
    s.push_str("kkkk");
    println!("{}", s);
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{},{},{}", s, part1, part2);

    let x = "abc";
    println!("{}", longest_with_an_announcement(x, "yy", "nidayede"));
}
use std::fmt::Display;

#[warn(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    } else {
        y
    }
}

fn guess_game() {
    let mut rng = rand::OsRng::new().expect("failed to create OsRng");

    println!("{:#?}", rng.gen::<bool>());

    let btm: i32 = 1;
    let top: i32 = 100;
    let secret = rng.gen_range::<i32>(btm, top);
    println!("secret is {:?}", secret);
    println!("guess the number");
    println!("Please input any number:");
    let mut guess;

    let mut guessnum: i32;
    let mut win: bool = false;
    while !win {
        guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        guessnum = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(e) => {
                println!("{}, str is {}", e, guess);
                continue;
            }
        };
        match guessnum.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("You win!");
                win = true;
            }
        }
    }
}
