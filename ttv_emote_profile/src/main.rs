extern crate reqwest;
extern crate scraper;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;

fn main() {
    download("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt",
        "2022-02-22.txt");
    download("https://cdn.destiny.gg/emotes/emotes.json", "emotes.json");

    let mut emotes: HashMap<String, i32> = HashMap::new();
    let mut users: HashMap<String, User> = HashMap::new();

    // create the initial list of emotes for each user and their count
    if let Ok(lines) = read_lines("./emotes.json") {
        for line in lines {
            if let Ok(message) = line {
                let emote: Vec<EmoteInfo> = serde_json::from_str(&message)
                    .expect("json not properly formatted");
                for each in emote.iter() {
                    emotes.insert(each.get_emote_name().to_string(), 0);
                }
            }
        }
    }

    // Go thru chat
    if let Ok(lines) = read_lines("./2022-02-22.txt") {
        for line in lines {
            if let Ok(message) = line {
                // Split message into username and message portion FIXME does dgg chat support non ASCII chars?
                let start_of_username_in_msg = message.find("] ").unwrap() + 2;
                let end_of_username_in_msg = message.find(": ").unwrap();
                let start_of_msg = message.find(": ").unwrap() + 2;
                let msg = &message[start_of_msg..];
                let username_in_msg = &message[start_of_username_in_msg..end_of_username_in_msg];

                // Check if username is unique relative to day
                if !users.contains_key(username_in_msg) {
                    users.insert(username_in_msg.to_string(),
                                 User {emote_use: emotes.clone(),
                                        username: username_in_msg.to_string()});
                }

                // Check emote usage of the message
                for (username, single_user) in users.iter_mut() {
                    for (emote, n) in single_user.emote_use.iter_mut() {
                        if msg.contains(emote) {
                            *n = *n + 1;
                            println!("{:?} has used {:?} {:?} times", username, emote, n);
                        }
                    }
                }
            }
        }
    }
}

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
    fn get_emote_name(&self) -> &str {
        &self.prefix
    }
}

impl User {
    fn get_username(&self) -> &str {
        &self.username
    }
}
