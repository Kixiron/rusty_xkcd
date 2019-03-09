extern crate chrono;
extern crate reqwest;
extern crate serde_json;
extern crate rand;

use super::Error;

/// # Comic
/// The data struct for xkcd comics
///
/// ## Contents
/// `title`: The title of the xkcd comic  
/// `url`: The url of the xkcd comic  
/// `img_url`: The url of the image of the xkcd comic  
/// `alt_text`: The alternative text or tooltip text oc the xkcd comic  
/// `number`: The number of the xkcd comic, raning from 0 to newest  
/// `date`: The date that the comic was published  
#[derive(Debug)]
#[allow(dead_code)]
pub struct Comic {
    pub title: String,                   // Title of the comic
    pub url: String,                     // Url of the comic
    pub img_url: String,                 // Image Url of the comic
    pub alt_text: String,                // Alt text or tooltip text of the comic
    pub number: i32,                     // Number of the comic
    pub date: chrono::Date<chrono::Utc>, // Date that the comic was published
}

impl Comic {
    /// # Get Comic
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

        // Request and return comic (Or any error that occured)
        match request_comic(&xkcd_url) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
    }
}

impl Comic {
    /// # Get Latest Comic
    /// Fetches the latest xkcd comic
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
}

// NOT WORKING
impl Comic {
    /// # Get Random Comic
    /// Fetches a random xkcd comic
    pub fn get_random_comic() -> Result<Comic, Error> {
        let comic_num: i32 = rand::random::<i32>();
        match request_comic(&format!("https://xkcd.com/{}/info.json", comic_num)) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        }
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
/// #         url: String::from("http://xkcd.com/589/"),
/// #         img_url: String::from("https://imgs.xkcd.com/comics/designated_drivers.png"),
/// #         alt_text: String::from("Calling a cab means cutting into beer money."),
/// #         number: 589,
/// #         date: Utc.ymd(2009, 5, 27),
/// #     })
/// # }
/// # // ADD PARSE JSON
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
        Err(e) => return Err(Error::RequestError(e.to_string())),
    };
    Ok(parse_comic(&body))
}

/// # Parse JSON
/// Parses the JSON from the xkcd API into a `Comic` struct
fn parse_comic(raw_json: &str) -> Comic { // Document This
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
    // Ghost comic_date into a Date
    let comic_date: Date<Utc> = Utc.ymd(
        comic_date.2.parse::<i32>().unwrap(),
        comic_date.0.parse::<u32>().unwrap(),
        comic_date.1.parse::<u32>().unwrap(),
    );

    // Build and return the comic
    Comic {
        title: value["title"].as_str().unwrap().to_string(),
        url: format!("http://xkcd.com/{}/", num),
        img_url: value["img"].as_str().unwrap().to_string(),
        alt_text: value["alt"].as_str().unwrap().to_string(),
        number: num,
        date: comic_date,
    }
}

/// # Get Latest Comic Number
/// Gets the number of the most recent xkcd comic
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
/// let latest_number: i32 = get_latest_comic_number().unwrap();
/// let latest_comic: Comic = get_latest_comic().unwrap();
/// assert_eq!(latest_number, latest_comic.number);
/// ````
fn get_latest_comic_number() -> Result<i32, Error> {
    let latest_comic_url: String = String::from("http://xkcd.com/info.0.json");
    match request_comic(&latest_comic_url) {
        Ok(data) => Ok(data.number),
        Err(e) => Err(e),
    }
}
