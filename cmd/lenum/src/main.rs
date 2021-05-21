#![allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Quit => Message::Quit,
            Message::Write(s) => Message::Write(s.clone()),
            Message::Move { x, y } => Message::Move { x: *x, y: *y },
            Message::ChangeColor(x, y, z) => Message::ChangeColor(*x, *y, *z),
        }
    }
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
        match self {
            Message::Quit => {
                println!("quit");
            }
            Message::Move { x, y } => {
                println!("Move -> {},{}", x, y)
            }
            Message::Write(s) => {
                println!("Write : {}", s)
            }
            Message::ChangeColor(a, b, c) => {
                println!("ChangeColor {}, {}, {}", a, b, c)
            }
        }
    }
}

fn mean<T>(v: &Vec<T>) -> Option<T>
where
    T: Copy + std::ops::Div + std::ops::AddAssign + std::ops::Div<Output = T> + From<usize>,
{
    if v.is_empty() {
        return Option::None;
    }
    let mut t: T = v[0];
    for val in v {
        t += *val;
    }
    let l: T = v.len().into();
    Option::Some(t / l)
}

fn median<T>(v: &Vec<T>) -> Option<T>
where
    T: Copy + std::ops::Div + std::ops::AddAssign + std::ops::Div<Output = T> + From<usize>,
{
    if v.is_empty() {
        return Option::None;
    }
    if v.len() % 2 == 1 {
        return Option::Some(v[v.len() / 2]);
    }
    let mut t = v[v.len() / 2];
    t += v[v.len() / 2 - 1];
    let dividor: T = 2.into();
    Option::Some(t / dividor)
}

fn mode<T>(v: &Vec<T>) -> Option<&T>
where
    T: Hash + Eq,
{
    if v.len() == 0 {
        return Option::None;
    }
    let mut mp: HashMap<&T, i32> = HashMap::new();

    for val in v {
        mp.insert(
            val,
            if let Option::Some(value) = mp.get(val) {
                value + 1
            } else {
                1
            },
        );
    }
    let mut mx = 0;
    let mut res: Option<&T> = Option::None;
    for (k, v) in mp {
        if mx < v {
            mx = v;
            res = Option::Some(k);
        }
    }
    return res;
}

fn main() {
    let s = String::from("नमस्ते");
    for x in s.chars() {
        println!("{}", x);
    }
    let mut it = s.char_indices().into_iter();
    loop {
        match it.next() {
            Option::None => {
                println!("end ");
                break;
            }
            Some(x) => match x {
                (a, b) => {
                    println!("{}, {}", a, b)
                }
            },
        }
    }

    println!("{}", s.len());
    let mut v = vec![1, 3, 3];
    v.push(2);
    let mut fourth = &mut v[3] as *mut usize;
    unsafe {
        *fourth = 6;
    }
    println!("{:?}", v);

    println!("mean is {:?}", mean(&v));
    println!("median is {:?}", median(&v));
    println!("mode is {:?}", mode(&v));

    let emp: Vec<usize> = vec![0];
    println!("vec len {}", emp.len());
    println!("mean is {:?}", mean(&emp));

    let m = Message::Write(String::from("hello"));
    m.call();
    let mv = Message::Move { x: 1, y: 3 };
    mv.call();
    let quit = Message::Quit.clone();
    quit.call();
    let q2 = quit.clone();
    quit.call();
    q2.call();
    let cc = Message::ChangeColor(1, 2, 3);
    cc.call();
}
