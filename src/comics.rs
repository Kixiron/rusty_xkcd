extern crate chrono;
extern crate rand;
extern crate reqwest;
extern crate serde_json;

use super::{Error, Explanation};

/// The struct containing all xkcd-comic related data and methods
///
/// ## Usage
/// There are three main ways to get a fully primed `Comic`
///
/// #### Get the comic by number
/// ```rust
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic = Comic::get_comic(100).unwrap(); // Get a comic by it's number
/// ```
/// However, getting a comic does have limits, as requesting a comic
/// that does not exist will throw an `InvalidNumber` error
/// ```should_panic
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic = Comic::get_comic(-1).unwrap(); // Too low!
/// ```
/// ```should_panic
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic = Comic::get_comic(999_999).unwrap(); // Too high!
/// ```
///
/// #### Get the latest comic
/// ```rust
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic = Comic::get_latest_comic().unwrap(); // Get the latest comic
/// ```
///
/// #### Get a random comic
/// ```rust
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic = Comic::get_random_comic().unwrap(); // Get a random comic
/// ```
///
/// ## Data
/// Data from the `Comic` struct can be extracted in one of two ways:
///
/// By 'dotting' the instance
/// ```rust
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic_number = Comic::get_random_comic().unwrap().number; // Get the comic's number
/// ```
///
/// Or by using one of the data methods
/// ```rust
/// # extern crate rusty_xkcd;
/// # use rusty_xkcd::Comic;
/// let comic_number = Comic::get_random_comic().unwrap().get_number(); // Get the latest comic's number
/// ```
///
/// ## Errors
/// There are two errors that can be thrown while acquiring a comic
///
/// #### Invalid Number
/// An invalid number error comes from your software or the end user requesting an xkcd comic with a
/// number that is either less than or equal to zero or greater than the newest xkcd comic's number.
/// For those who speak code more fluently than english, here's a snippet:
/// ```rust
/// # let input_number = 0;
/// # let latest_comic_number = 0;
/// # fn throw_error() {
/// #     println!("Thanks for looking at the source code!");
/// # }
/// if input_number <= 0 || input_number > latest_comic_number {
///     throw_error();
/// }
/// ```
///
/// #### Request Error
/// A request error can happen for any number of reasons, but all are related to some sort of failure
/// in querying the xkcd api
#[derive(Debug)]
#[allow(dead_code)]
pub struct Comic {
    /// Title of the comic
    pub title: String,
    /// Url of the comic `https://xkcd.com/{comic_number}`
    pub url: String,
    /// Image Url of tht comic `https://imgs.xkcd.com/comics/{image_name}.png`
    pub img_url: String,
    /// Alt text or tooltip text of the comic
    pub alt_text: String,
    /// Number of the comic
    pub number: i32,
    /// Date that the comic was published
    pub date: chrono::Date<chrono::Utc>,
}

impl Comic {
    /// Fetches the chosen xkcd comic via `i32`
    ///
    /// ## Usage
    /// ```rust
    /// # use rusty_xkcd::Comic;
    /// let comic: Comic = Comic::get_comic(100).unwrap(); // Get xkcd comic number 100
    /// println!("{:?}", comic);
    /// println!("{}", comic.number);
    /// println!("{}", comic.url);
    /// ```
    pub fn get_comic(comic_num: i32) -> Result<Comic, Error> {
        // If requested comic's number is less than or equal to zero, error
        if comic_num <= 0 {
            Err(Error::InvalidNumber(comic_num))?;
        }

        // Get newest comic's number
        let newest_comic_num: i32 = get_latest_comic_number().unwrap();

        // If the requested number is greater than the newest or lower than zero, error
        if comic_num > newest_comic_num {
            Err(Error::InvalidNumber(comic_num))?;
        }

        let xkcd_url: String = format!("http://xkcd.com/{}/info.0.json", comic_num); // Form url

        // Request and return comic (Or any error that occurred)
        match request_comic(&xkcd_url) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    /// Fetches the latest xkcd comic.
    ///
    /// ## Usage
    /// ```rust
    /// # use rusty_xkcd::Comic;
    /// let comic: Comic = Comic::get_latest_comic().unwrap(); // Get the latest xkcd comic
    /// println!("{:?}", comic);
    /// println!("{}", comic.number);
    /// println!("{}", comic.url);
    /// ```
    pub fn get_latest_comic() -> Result<Comic, Error> {
        // Request and return the newest comic
        match request_comic("http://xkcd.com/info.0.json") {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }

    /// Fetches a random xkcd comic
    ///
    /// ## Usage
    /// ```rust
    /// # use rusty_xkcd::Comic;
    /// let comic: Comic = Comic::get_random_comic().unwrap(); // Get a random xkcd comic
    /// println!("{:?}", comic);
    /// println!("{}", comic.number);
    /// println!("{}", comic.url);
    /// ```
    pub fn get_random_comic() -> Result<Comic, Error> {
        use rand::prelude::*;

        let latest_comic: i32 = get_latest_comic_number().unwrap();
        let comic_num: i32 = thread_rng().gen_range(1, latest_comic);

        Comic::get_comic(comic_num)
    }

    /// Fetches the `Explanation` of the current comic
    pub fn explain(&self) -> Result<Explanation, Error> {
        Explanation::explain(self.number)
    }

    /// Fetches the current comic's title
    pub fn get_title(&self) -> String {
        let x: String = (*self.title).to_string();
        x
    }

    /// Fetches the current comic's url
    pub fn get_url(&self) -> String {
        let x: String = (*self.url).to_string();
        x
    }

    /// Fetches the current comic's image url
    pub fn get_img_url(&self) -> String {
        let x: String = (*self.img_url).to_string();
        x
    }

    /// Fetches the current comic's alt/tooltip text
    pub fn get_alt_text(&self) -> String {
        let x: String = (*self.alt_text).to_string();
        x
    }

    /// Fetches the current comic's number
    pub fn get_number(&self) -> i32 {
        self.number
    }

    /// Fetches the current comic's date
    pub fn get_date(&self) -> chrono::Date<chrono::Utc> {
        self.date
    }
}

/// # Request Comic
/// Requests a comic via formed url
///
/// ## Usage
/// ```rust
/// # use chrono::prelude::*;
/// # use rusty_xkcd::{ Comic, Error };
/// # fn request_comic(url: &str) -> Result<Comic, Error> {
/// #     Ok(Comic {
/// #         title: String::from("Designated Drivers"),
/// #         url: String::from("http://xkcd.com/589"),
/// #         img_url: String::from("https://imgs.xkcd.com/comics/designated_drivers.png"),
/// #         alt_text: String::from("Calling a cab means cutting into beer money."),
/// #         number: 589,
/// #         date: Utc.ymd(2009, 5, 27),
/// #     })
/// # }
/// let comic_one: Comic = request_comic("https://xkcd.com/589/info.0.json").unwrap(); // Get the 100'th xkcd comic
///
/// let url: String = String::from("https://xkcd.com/589/info.json"); // Form the url for the xkcd comic
/// let comic_two: Comic = request_comic(&url).unwrap(); // Get the 100'th xkcd comic
///
/// assert_eq!(format!("{:?}", comic_one), format!("{:?}", comic_two));
/// ```
fn request_comic(url: &str) -> Result<Comic, Error> {
    let body: String = match reqwest::get(url) {
        Ok(mut res) => res.text().unwrap(),
        Err(e) => Err(Error::RequestError(e.to_string()))?,
    };
    Ok(parse_comic(&body))
}

/// # Parse JSON
/// Parses the JSON from the xkcd API into a `Comic` struct
fn parse_comic(raw_json: &str) -> Comic {
    // Document This
    use chrono::prelude::*;

    // Desearilize the JSON from xkcd API
    let value: serde_json::Value = serde_json::from_str(raw_json).unwrap();
    let num: i32 = value["num"].to_string().parse::<i32>().unwrap();

    // Parse Month, Day and Year into strings
    let comic_date: (String, String, String) = (
        value["month"].as_str().unwrap().to_string(),
        value["day"].as_str().unwrap().to_string(),
        value["year"].as_str().unwrap().to_string(),
    );
    // Shadow comic_date into a Date
    let comic_date: Date<Utc> = Utc.ymd(
        comic_date.2.parse::<i32>().unwrap(),
        comic_date.0.parse::<u32>().unwrap(),
        comic_date.1.parse::<u32>().unwrap(),
    );

    // Build and return the comic
    Comic {
        title: value["title"].as_str().unwrap().to_string(),
        url: format!("http://xkcd.com/{}", num),
        img_url: value["img"].as_str().unwrap().to_string(),
        alt_text: value["alt"].as_str().unwrap().to_string(),
        number: num,
        date: comic_date,
    }
}

/// # Get Latest Comic Number
/// Gets the number of the most recent xkcd comic.
///
/// ## Usage
///
/// ```rust
/// # use rusty_xkcd::{ Error, Comic };
/// # use chrono::prelude::*;
/// # fn get_latest_comic() -> Result<Comic, Error> {
/// #     Ok(Comic {
/// #         title: String::from("Designated Drivers"),
/// #         url: String::from("http://xkcd.com/589/"),
/// #         img_url: String::from("https://imgs.xkcd.com/comics/designated_drivers.png"),
/// #         alt_text: String::from("Calling a cab means cutting into beer money."),
/// #         number: 589,
/// #         date: Utc.ymd(2009, 5, 27),
/// #     })
/// # }
/// # fn get_latest_comic_number() -> Result<i32, Error> {
/// #     Ok(589)
/// # }
/// let latest_number: i32 = get_latest_comic_number().unwrap(); // Get the latest comic number
/// let latest_comic: Comic = get_latest_comic().unwrap(); // Get the latest comic
/// assert_eq!(latest_number, latest_comic.number);
/// ````
fn get_latest_comic_number() -> Result<i32, Error> {
    match request_comic("http://xkcd.com/info.0.json") {
        Ok(data) => Ok(data.number),
        Err(e) => Err(e),
    }
}
