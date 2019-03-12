extern crate reqwest;
extern crate scraper;

use super::{Comic, Error};

// Display url: https://www.explainxkcd.com/wiki/index.php/{}
// Random url: https://www.explainxkcd.com/wiki/index.php/Special:Random
// Api url: https://www.explainxkcd.com/wiki/api.php?action=parse&format=json&pageid={}&utf8=1

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
        fetch_explanation(num)
    }

    pub fn get_comic(&self) -> Result<Comic, Error> {
        Comic::get_comic(self.xkcd_num)
    }

    pub fn get_explanation(&self) -> String {
        let x: String = (*self.explanation).to_string();
        x
    }
    
    pub fn get_explanation_url(&self) -> String {
        let x: String = (*self.explanation_url).to_string();
        x
    }

    pub fn get_xkcd_url(&self) -> String {
        let x: String = (*self.xkcd_url).to_string();
        x
    }

    pub fn get_xkcd_number(&self) -> i32 {
        self.xkcd_num
    }
}

fn fetch_explanation(num: i32) -> Result<Explanation, Error> {
    let url: String = format!("https://www.explainxkcd.com/wiki/index.php/{}", num);
    let body: String = match reqwest::get(&url) {
        Ok(mut res) => res.text().unwrap(),
        Err(e) => Err(Error::RequestError(e.to_string()))?,
    };
    parse_html(&body)
}

fn parse_html(html: &str) -> Result<Explanation, Error> {
    Ok(Explanation {
        explanation: String::from("test"),
        explanation_url: String::from("test"),
        xkcd_url: String::from("test"),
        xkcd_num: 10,
    })
}
