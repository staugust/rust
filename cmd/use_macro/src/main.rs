use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::mem::size_of_val;

#[derive(HelloMacro)]
struct MyStruct{
    a : i32,
    c : i32,
    b : i64,
}

fn main() {
    println!("{}", size_of_val(&MyStruct{a: 4, b : 5, c: 3,}));
    MyStruct::hello_macro();
    println!("Hello, world!");
}
