extern crate chrono;
extern crate reqwest;
extern crate serde_json;

#[macro_use]
extern crate quick_error;

pub fn get_comic(comic_num: i32) -> Result<XkcdData, XkcdError> {
    if comic_num < 0 {
        Err(XkcdError::InvalidNumber(comic_num))?;
    }

    // Get newest comic's number
    let newest_comic_num: i32 = get_newest_comic_num().unwrap();

    // If the requested number is greater than the newest or lower than zero, error
    if comic_num > newest_comic_num {
        Err(XkcdError::InvalidNumber(comic_num))?;
    }

    let xkcd_url: String = format!("http://xkcd.com/{}/info.0.json", comic_num); // Form url

    // Request and return comic
    match request_xkcd(xkcd_url) {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

pub fn get_latest_comic() -> Result<XkcdData, XkcdError> {
    // Request and return the newest comic
    match request_xkcd(String::from("http://xkcd.com/info.0.json")) {
        Ok(data) => Ok(data),
        Err(e) => Err(e),
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct XkcdData {
    pub title: String,                   // Title of the comic
    pub comic_url: String,               // Url of the comic
    pub img_url: String,                 // Image Url of the comic
    pub alt_text: String,                // Alt text or tooltip text of the comic
    pub number: i32,                     // Number of the comic
    pub date: chrono::Date<chrono::Utc>, // Date that the comic was published
}

fn request_xkcd(url: String) -> Result<XkcdData, XkcdError> {
    let body: String = match reqwest::get(&url) {
        Ok(mut res) => res.text().unwrap(),
        Err(e) => return Err(XkcdError::RequestError(e.to_string())),
    };
    Ok(parse_json(&body))
}

fn parse_json(raw_json: &str) -> XkcdData {
    use chrono::prelude::*;

    let value: serde_json::Value = serde_json::from_str(raw_json).unwrap();
    let num: i32 = value["num"].to_string().parse::<i32>().unwrap();

    let date: (String, String, String) = (
        value["month"].as_str().unwrap().to_string(),
        value["day"].as_str().unwrap().to_string(),
        value["year"].as_str().unwrap().to_string(),
    );
    let date: Date<Utc> = Utc.ymd(
        date.2.parse::<i32>().unwrap(),
        date.0.parse::<u32>().unwrap(),
        date.1.parse::<u32>().unwrap(),
    );

    XkcdData {
        title: value["title"].as_str().unwrap().to_string(),
        comic_url: format!("http://xkcd.com/{}/", num),
        img_url: value["img"].as_str().unwrap().to_string(),
        alt_text: value["alt"].as_str().unwrap().to_string(),
        number: num,
        date,
    }
}

fn get_newest_comic_num() -> Result<i32, XkcdError> {
    let latest_comic_url: String = String::from("http://xkcd.com/info.0.json");
    match request_xkcd(latest_comic_url) {
        Ok(data) => Ok(data.number),
        Err(e) => Err(e),
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum XkcdError {
        InvalidNumber(num: i32) {
            description("Invalid xkcd num")
            display(r#"Invalid xkcd num: {}"#, num)
        }

        RequestError(error: String) {
            description("Xkcd request Error")
            display(r#"Xkcd request error: {}"#, error)
        }

        DeserializeError(error: String) {
            description("Deserialization Error")
            display(r#"Deserialization error: {}"#, error)
        }
    }
}
