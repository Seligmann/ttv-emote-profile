extern crate reqwest;
extern crate scraper;

use std::io;
use std::fs::File;

fn main() {
    download("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt");
}

/*
FIXME add automatic parsing of url to get the date, which will be the name of the file that is
created
*/

fn download(url: &str) {
    let url = String::from(url);
    let resp = reqwest::blocking::get(url).expect("Failed to get url");
    let body = resp.text().expect("Body is invalid");
    let mut out = File::create("2022-02.22-txt").expect("Failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
}
