extern crate reqwest;
extern crate scraper;

use std::io;
use std::fs::File;

fn main() {
    let resp = reqwest::blocking::get("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt")
        .expect("request failed");
    let body = resp.text().expect("body invalid");
    let mut out = File::create("2022-02-22.txt")
        .expect("failed to create file");
    io::copy(&mut body.as_bytes(), &mut out)
        .expect("failed to copy content");
}
