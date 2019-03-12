extern crate scraper;

use super::{Comic, Error};

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
}

impl Explanation {
    pub fn explain(url: &str) -> Result<Explanation, Error> {
        fetch_explanation(&url)
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

//////////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait Explain {
    fn explain(&self) -> Result<Explanation, Error>;
}

impl Explain for Comic {
    fn explain(&self) -> Result<Explanation, Error> {
        fetch_explanation(&self.url)
    }
}
