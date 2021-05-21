use lazy_static::lazy_static;

lazy_static! {
    static ref VALUE: i64 = {
        let mut x = 5;
        x *= x;
        x
    };
}

pub fn p(s: String) {
    let addr = &VALUE as *const VALUE as usize;
    println!("{} -> {}", s, addr)
}
