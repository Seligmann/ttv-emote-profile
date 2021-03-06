/*
Use of main.rs requires OverRustleLogs to have previously been scraped into a directory of the name
"dgg-logs". The location of /dgg-logs/ should be the parent of the /src/ directory by 1 layer,
e.g. ~/.../dgg-logs/ttv_emote_profile/src/". OverRustleLogs can easily be scraped with
https://github.com/Seligmann/OverrustleScraper .

It is also required that the current list of valid emotes is known. To download this insert the
following on line 1 of main():

download("https://cdn.destiny.gg/emotes/emotes.json", "emotes.json");

 */

use std::io::{self, BufRead};
use std::fs::{self, File, DirEntry};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::path;
use select::document::Document;
use select::predicate::Name;
use serde::Deserialize;
use reqwest;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut valid_emotes: HashSet<String> = HashSet::new();
    let mut users: HashMap<String, User> = HashMap::new();
    let mut init_emote_list: HashMap<String, i32> = HashMap::new();

    // create the initial list of emotes for each user and their count
    if let Ok(lines) = read_lines("./emotes.json") {
        for line in lines {
            if let Ok(message) = line {
                let emote: Vec<EmoteInfo> = serde_json::from_str(&message)
                    .expect("json not properly formatted");
                for each in emote.iter() {
                    valid_emotes.insert(each.prefix.to_string());
                    init_emote_list.insert(each.prefix.to_string(), 0);
                }
            }
        }
    }

    // Go through each chat log day
    let paths = fs::read_dir("../dgg-logs").unwrap();
    for file in paths {
        let file = file.unwrap().path().display().to_string();
        if let Ok(lines) = read_lines(file) {
            for line in lines {
                if let Ok(message) = line {
                    // Split message into username and message portion
                    let start_of_username_in_msg = message.find("] ").unwrap() + 2;
                    let end_of_username_in_msg = message.find(": ").unwrap();
                    let start_of_msg = message.find(": ").unwrap() + 2;
                    let msg = &message[start_of_msg..];
                    let username_in_msg = &message[start_of_username_in_msg..end_of_username_in_msg];

                    // Check if username is unique relative to day
                    if !users.contains_key(username_in_msg) {
                        users.insert(username_in_msg.to_string(),
                                     User {emote_use: init_emote_list.clone(),
                                         username: username_in_msg.to_string()});
                    }

                    // Check emote usage in the message
                    let mut used_emotes = HashSet::new();
                    for emote in valid_emotes.iter() {
                        if msg.contains(emote) && !used_emotes.contains(emote) {
                            used_emotes.insert(emote.to_string());
                            *users
                                .get_mut(username_in_msg)
                                .unwrap()
                                .emote_use
                                .get_mut(emote)
                                .unwrap() += 1;

                            println!("{:?} used {:?} {:?} times",
                                     username_in_msg,
                                     emote,
                                     *users
                                         .get_mut(username_in_msg)
                                         .unwrap()
                                         .emote_use
                                         .get_mut(emote)
                                         .unwrap()
                            )
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
