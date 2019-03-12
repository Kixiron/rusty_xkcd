extern crate scraper;

struct Explanation {
    pub explanation: String,
    pub explanation_url: String,
    pub xkcd_url: String,
    xkcd_num: i32,
}

impl Explanation {
    pub fn get_xkcd(&self) -> rusty_xkcd::Comic {
        rusty_xkcd::Comic::get_comic(xkcd_num)
    }
}

impl Explanation {
    pub fn get_explanation(input: Option<str, String, &str, Comic>) -> Explanation {
        match input {
            str | &str | String => fetch_explanation(input),
            Comic => fetch_explanation(Comic.url),
        }
    }
}

fn fetch_explanation(url: &str) -> Explanation {
    Explanation {
        explanation: String::from("test"),
        explanation_url: String::from("test"),
        xkcd_url: String::from("test"),
    }
}

fn parse_html(html: &str) -> ! {

}
