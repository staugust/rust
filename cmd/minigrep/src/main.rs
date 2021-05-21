use std::env;
use std::process;

unsafe fn dangerous() {
    println!("oh, dangerous!!!")
}

fn pp(mut k: i32) {
    println!("before unsafe mute {}", k);
    unsafe {
        let x: *mut i32 = &mut k as *mut i32;
        *x = 500;
        println!("unsafe print {}", *x)
    }
    println!("after unsafe mute {}", k)
}

#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    unsafe {
        dangerous();
    }

    pp(5);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let mut x = 5;
    let y = &mut x;
    println!(" {}", *y);
    *y = 10;
    println!(" {}", *y);
    *y = 20;
    println!(" {}", *y);
    println!(" {:}", y);
    *y = *y + 5;
    println!(" {:}", y);
    let m = String::from("bc");
    let s = &m;
    let f = |s: &str| println!("{}", s);
    f(s);
    let args: Vec<String> = env::args().collect();
    let config = minigrep::Config::new(&args).unwrap_or_else(|err| {
        println!("problem in parsing arguments: {}", err);
        process::exit(-1);
    });
    let result = minigrep::run(config);
    if let Err(e) = result {
        println!("Application error: {:}", e);
        process::exit(-2);
    }
}
