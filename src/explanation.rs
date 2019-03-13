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
        (*self.explanation).to_string()
    }

    pub fn get_explanation_url(&self) -> String {
        (*self.explanation_url).to_string()
    }

    pub fn get_xkcd_url(&self) -> String {
        (*self.xkcd_url).to_string()
    }

    pub fn get_xkcd_number(&self) -> i32 {
        self.xkcd_num
    }
}

fn fetch_explanation(num: i32) -> Result<Explanation, Error> {
    let url: String = format!("https://www.explainxkcd.com/wiki/index.php/{}", num);
    let body: String = match reqwest::get(&url) {
        Ok(mut res) => res.text().unwrap(),
        Err(e) => Err(Error::Request::new(&e.to_string()))?,
    };
    println!("{:?}", body);
    Ok(parse_html(&body, num))
}

fn parse_html(html: &str, num: i32) -> Explanation {
    use scraper::*;

    let parsed_html: Html = Html::parse_document(html);

    let explanation: String = {
        // Fetch explanation
        String::from("Placeholder")
    };
    let explanation_url: String = format!("https://www.explainxkcd.com/wiki/index.php/{}", num);
    let xkcd_url: String = format!("https://xkcd.com/{}", num);
    let xkcd_num: i32 = num;

    Explanation {
        explanation,
        explanation_url,
        xkcd_url,
        xkcd_num,
    }
}
