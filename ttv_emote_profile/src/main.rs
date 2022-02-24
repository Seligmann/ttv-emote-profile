extern crate reqwest;
extern crate scraper;

use std::io::{self, BufRead, Read};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::ops::Deref;
use serde::Deserialize;


fn main() {
    download("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt",
        "2022-02-22.txt");
    download("https://cdn.destiny.gg/emotes/emotes.json", "emotes.json");

    if let Ok(lines) = read_lines("./emotes.json") {
        for line in lines {
            if let Ok(message) = line {
                let emote: Vec<EmoteInfo> = serde_json::from_str(&message)
                    .expect("json not properly formatted");
                for each in emote.iter() {
                    println!("{:?}", each.mut_get());
                }
            }
        }
    }

    // Go thru chat
    if let Ok(lines) = read_lines("./2022-02-22.txt") {
        for line in lines {
            if let Ok(message) = line {
                // do something with message
            }
        }
    }
}

/*
FIXME add automatic parsing of url to get the date, which will be the name of the file that is
created
*/

fn download(url: &str, name: &str) {
    let url = String::from(url);
    let resp = reqwest::blocking::get(url).expect("Failed to get url");
    let body = resp.text().expect("Body is invalid");
    let mut out = File::create(name).expect("Failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
    where T: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct User {
    emote_use: HashMap<String, i32>,
    username: String
}

#[derive(Debug, Deserialize)]
struct EmoteInfo{
    image: Vec<ImageInfo>,
    prefix: String,
    theme: String,
    twitch: bool
}

#[derive(Debug, Deserialize)]
struct ImageInfo {
    height: i32,
    mime: String,
    name: String,
    url: String,
    width: i32
}

impl EmoteInfo {
    fn mut_get(&self) -> &str{
        &self.prefix
    }
}
