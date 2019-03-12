quick_error! {
    #[derive(Debug)]
    pub enum Error {

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
        ///     _ => println!("{}", Error::InvalidNumber(number)),
        /// }
        /// ```
        InvalidNumber(number: i32) {
            description("Invalid xkcd num")
            display(r#"Invalid xkcd num: {}"#, number)
        }

        /// ### Request Error
        /// This error is thrown when a request fails
        ///
        /// #### Usage
        /// ```rust
        /// # use rusty_xkcd::Error;
        /// # fn make_request() -> Result<String, Error> {
        /// #     Ok(String::from("Sucessful Request!"))
        /// # }
        /// match make_request() { // Example request function
        ///     Ok(data) => println!("{}", data),
        ///     Err(e) => println!("{}", Error::RequestError(e.to_string())),
        /// }
        /// ```
        RequestError(error: String) { // TODO: Add &str reference with lifetime
            description("Xkcd request Error")
            display(r#"Xkcd request error: {}"#, error)
        }
    }
}
