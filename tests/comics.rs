extern crate chrono;
extern crate rusty_xkcd;

use rusty_xkcd::*;

#[test]
fn get_comic_test() {
    use chrono::prelude::*;

    // Aquire test comic
    let test_comic: Comic = Comic::get_comic(589).unwrap();
    // Create control comic
    let control_comic: Comic = Comic {
        title: String::from("Designated Drivers"),
        url: String::from("http://xkcd.com/589/"),
        img_url: String::from("https://imgs.xkcd.com/comics/designated_drivers.png"),
        alt_text: String::from("Calling a cab means cutting into beer money."),
        number: 589,
        date: Utc.ymd(2009, 5, 27),
    };

    // Make sure all fields are the same
    assert_eq!(test_comic.title, control_comic.title);
    assert_eq!(test_comic.url, control_comic.url);
    assert_eq!(test_comic.img_url, control_comic.img_url);
    assert_eq!(test_comic.alt_text, control_comic.alt_text);
    assert_eq!(test_comic.number, control_comic.number);
    assert_eq!(test_comic.date, control_comic.date);
}

#[test]
fn get_latest_comic_test() {
    // Fetch the latest comic twice to see if they are the same
    let comic_one: Comic = Comic::get_latest_comic().unwrap();
    let comic_two: Comic = Comic::get_latest_comic().unwrap();

    // Make sure all fields are the same
    assert_eq!(comic_one.title, comic_two.title);
    assert_eq!(comic_one.url, comic_two.url);
    assert_eq!(comic_one.img_url, comic_two.img_url);
    assert_eq!(comic_one.alt_text, comic_two.alt_text);
    assert_eq!(comic_one.number, comic_two.number);
    assert_eq!(comic_one.date, comic_two.date);
}

/*
DISABLED UNTIL FIXED
#[test]
fn get_random_comic_test() {
    let random_comic: Comic = Comic::get_random_comic().unwrap();
    println!("{:?}", random_comic);
    let control_comic: Comic = Comic::get_comic(random_comic.number).unwrap();

    // Make sure all fields are the same
    assert_eq!(random_comic.title, control_comic.title);
    assert_eq!(random_comic.url, control_comic.url);
    assert_eq!(random_comic.img_url, control_comic.img_url);
    assert_eq!(random_comic.alt_text, control_comic.alt_text);
    assert_eq!(random_comic.number, control_comic.number);
    assert_eq!(random_comic.date, control_comic.date);
}
*/

#[test]
#[should_panic]
/// Test for an absurdly high numbered comic, should fail
fn numbered_comic_higher_test() {
    let _test_comic: Comic = Comic::get_comic(999_999_999).unwrap();
}

#[test]
#[should_panic]
/// Test for a negative numbered comic, should fail
fn numbered_comic_lower_test() {
    let _test_comic: Comic = Comic::get_comic(-1).unwrap();
}
