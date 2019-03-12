extern crate scraper;

struct Explination {
    pub explination: String,
    pub explination_url: String,
    pub xkcd_url: String,
    xkcd_num: i32,
}

impl Explination {
    pub fn get_xkcd(&self) -> rusty_xkcd::Comic {
        rusty_xkcd::Comic::get_comic(xkcd_num)
    }
}

impl Explination {
    pub fn get_explination(input: Option<str, String, &str, Comic>) -> Explination {
        match input {
            str | &str | String => fetch_explination(input),
            Comic => fetch_explination(Comic.url),
        }
    }
}

fn fetch_explination(url: &str) -> Explination {
    Explination {
        explination: String::from("test"),
        explination_url: String::from("test"),
        xkcd_url: String::from("test"),
    }
}

fn parse_html(html: &str) -> ! {

}
