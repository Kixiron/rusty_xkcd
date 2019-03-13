// quick_error! {
// #[derive(Debug)]
// pub enum Error {
//
// InvalidNumber(number: i32) {
// description("Invalid xkcd num")
// display(r#"Invalid xkcd num: {}"#, number)
// }
//
//
// RequestError(error: String) {
// description("Xkcd request Error")
// display(r#"Xkcd request error: {}"#, error)
// }
// }
// }
//

use std::fmt::{Display, Formatter, Result};

/// The module containing all errors for rusty_xkcd
pub enum Error {
    Number,
    Request,
}

/// ### Invalid Number
/// This error is thrown when the requested item's number is out of range
///
/// #### Usage
/// ```rust
/// # use rusty_xkcd::Error;
/// # fn get_number() -> i32 {
/// #     10
/// # }
/// let number: i32 = get_number(); // Example function that returns an int
/// match number {
///     1 => println!("The number is one!"),
///     _ => println!("{}", Error::Number::new(number)),
/// }
/// ```
#[derive(Debug)]
pub struct Number {
    pub number: i32,
}
impl Number {
    pub fn new(num: i32) -> Number {
        Number { number: num }
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Invalid number: {}", self.number)
    }
}

/// ### Request Error
/// This error is thrown when a request fails
///
/// #### Usage
/// ```rust
/// # use rusty_xkcd::Error;
/// # fn make_request() -> Result<String, Error> {
/// #     Ok(String::from("Successful Request!"))
/// # }
/// match make_request() {
///     // Example request function
///     Ok(data) => println!("{}", data),
///     Err(e) => println!("{}", Error::Request::new(e.to_string())),
/// }
/// ```
#[derive(Debug)]
pub struct Request {
    pub number: i32,
}
impl Request {
    pub fn new(num: i32) -> Request {
        Request { number: num }
    }
}
impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.number)
    }
}
