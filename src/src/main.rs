extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
use reqwest::blocking;

fn main() {
    over_rustle("https://news.ycombinator.com");
}

fn over_rustle(url: &str) {
    let mut resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));
}