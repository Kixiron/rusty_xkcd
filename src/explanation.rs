extern crate scraper;

use super::{Comic, Error, InvertExplanation};

#[derive(Debug)]
pub struct Explanation {
    pub explanation: String,
    pub explanation_url: String,
    pub xkcd_url: String,
    pub xkcd_num: i32,
}

impl Explanation {
    pub fn get_xkcd(&self) -> Result<Comic, Error> {
        Comic::get_comic(self.xkcd_num)
    }

    pub fn explain(num: i32) -> Result<Explanation, Error> {
        fetch_explanation(&url)
    }
}

impl InvertExplanation for Explanation {
    fn get_comic(&self) -> Result<Comic, Error> {
        Comic::get_comic(self.xkcd_num)
    }
}

fn fetch_explanation(url: &str) -> Result<Explanation, Error> {
    // Get html
    parse_html("test")
}

fn parse_html(html: &str) -> Result<Explanation, Error> {
    Ok(Explanation {
        explanation: String::from("test"),
        explanation_url: String::from("test"),
        xkcd_url: String::from("test"),
        xkcd_num: 10,
    })
}
