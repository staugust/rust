/// Generally, the first line is a brief summary describing the function.
///
/// The next lines present detailed documentation.
/// Code blocks start with triple backticks. The code has an implicit `fn main()` inside and `extern crate <cratename>`,
/// which means you can just start writing code.
///
/// ```
/// let result = testcase::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}


/// This function divides two numbers.
///
/// # Example #1: 10 / 2 == 5
///
/// ```
/// let result = testcase::div(10, 2);  // TODO: finish this test!
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2: 6 / 2 = 3
///
/// ```
/// let result = testcase::div(6, 2);
/// assert_eq!(result, 3);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// #[should_panic]
/// testcase::div(3, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1: 9 - 2 == 7
///
/// ```
/// assert_eq!(testcase::sub(9, 2), 7);
/// ```
///
/// # Example #2: 6 - 9 == -3
///
/// ```
/// assert_eq!(testcase::sub(6, 9), -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}