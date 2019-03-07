extern crate chrono;
extern crate rusty_xkcd;

#[cfg(test)]
mod tests {

    #[test]
    fn get_comic_test() {
        use chrono::prelude::*;

        let test_comic: rusty_xkcd::XkcdData = rusty_xkcd::get_comic(589).unwrap();
        let control_comic: rusty_xkcd::XkcdData = rusty_xkcd::XkcdData {
            title: String::from("Designated Drivers"),
            comic_url: String::from("http://xkcd.com/589/"),
            img_url: String::from("https://imgs.xkcd.com/comics/designated_drivers.png"),
            alt_text: String::from("Calling a cab means cutting into beer money."),
            number: 589,
            date: Utc.ymd(2009, 5, 27),
        };

        assert_eq!(test_comic.title, control_comic.title);
        assert_eq!(test_comic.comic_url, control_comic.comic_url);
        assert_eq!(test_comic.img_url, control_comic.img_url);
        assert_eq!(test_comic.alt_text, control_comic.alt_text);
        assert_eq!(test_comic.number, control_comic.number);
        assert_eq!(test_comic.date, control_comic.date);
    }

    #[test]
    #[should_panic]
    fn comic_higher_than_bounds_test() {
        let _test_comic: rusty_xkcd::XkcdData = rusty_xkcd::get_comic(999999999).unwrap();
    }

    #[test]
    #[should_panic]
    fn comic_lower_than_bounds_test() {
        let _test_comic: rusty_xkcd::XkcdData = rusty_xkcd::get_comic(-1).unwrap();
    }
}
