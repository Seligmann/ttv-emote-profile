use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::{HashMap, HashSet};
// use scraper::Node::Document;
use select::document::Document;
use select::predicate::Name;
use serde::Deserialize;
use reqwest;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    scrape_overrustle();

    download("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt",
        "2022-02-22.txt");
    download("https://cdn.destiny.gg/emotes/emotes.json", "emotes.json");

    let mut valid_emotes: HashSet<String> = HashSet::new();
    let users: HashMap<String, User> = HashMap::new();
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

    // Go thru chat
    // if let Ok(lines) = read_lines("./2022-02-22.txt") {
    //     for line in lines {
    //         if let Ok(message) = line {
    //             // Split message into username and message portion FIXME does dgg chat support non ASCII chars?
    //             let start_of_username_in_msg = message.find("] ").unwrap() + 2;
    //             let end_of_username_in_msg = message.find(": ").unwrap();
    //             let start_of_msg = message.find(": ").unwrap() + 2;
    //             let msg = &message[start_of_msg..];
    //             let username_in_msg = &message[start_of_username_in_msg..end_of_username_in_msg];
    //
    //             // Check if username is unique relative to day
    //             if !users.contains_key(username_in_msg) {
    //                 users.insert(username_in_msg.to_string(),
    //                              User {emote_use: init_emote_list.clone(), // fixme ownership issue?
    //                                     username: username_in_msg.to_string()});
    //             }
    //
    //             // Check emote usage in the message
    //             let mut used_emotes = HashSet::new();
    //             for emote in valid_emotes.iter() {
    //                 if msg.contains(emote) && !used_emotes.contains(emote) {
    //                     used_emotes.insert(emote.to_string());
    //                     *users
    //                         .get_mut(username_in_msg)
    //                         .unwrap()
    //                         .emote_use
    //                         .get_mut(emote)
    //                         .unwrap() += 1;
    //
    //                     // println!("{:?} used {:?} {:?} times",
    //                              username_in_msg,
    //                              emote,
    //                              *users
    //                                  .get_mut(username_in_msg)
    //                                  .unwrap()
    //                                  .emote_use
    //                                  .get_mut(emote)
    //                                  .unwrap()
    //                     )
    //                 }
    //             }
    //         }
    //     }
    // }
}

fn download(url: &str, name: &str) {
    let url = String::from(url);
    let resp = reqwest::blocking::get(url).expect("Failed to get url");
    let body = resp.text().expect("Body is invalid");
    let mut out = File::create(name).expect("Failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
}

fn scrape_overrustle() {
    // https://overrustlelogs.net/Destinygg%20chatlog
    // https://overrustlelogs.net/Destinygg%20chatlog/January%202022
    // https://overrustlelogs.net/Destinygg%20chatlog/January%202022/2022-01-25
    let bad_hrefs = vec!["userlogs", "broadcaster", "subscribers", "bans", "top100"];
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    let months_map = HashMap::from([
        ("January", "01"),
        ("February", "02"),
        ("March", "03"),
        ("April", "04"),
        ("May", "05"),
        ("June", "06"),
        ("July", "07"),
        ("August", "08"),
        ("September", "09"),
        ("October", "10"),
        ("November", "11"),
        ("December", "12"),
    ]);

    let mut years: Vec<i32> = Vec::new();
    for n in 2013..2070 {
        years.push(n);
    }

    let url = "https://overrustlelogs.net/Destinygg%20chatlog";
    let chatlog = "/Destinygg chatlog/";
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
                if link.contains(chatlog) {
                    for month in months.iter() {
                        if link.contains(month) {
                            for year in &years {
                                if link.contains(&year.to_string()) { // "/Destinygg chatlog/June 2015"
                                    // println!("{:?}", link);
                                    /*
                                    month and year of each msg is now known; scrape for each txt file
                                     */
                                    let mut url: String = "https://overrustlelogs.net/Destinygg%20chatlog".to_owned();
                                    let month_year: String = "/".to_owned() + *month + &*"%20".to_owned() + &*year.to_string();
                                    url.push_str(&month_year);
                                    // println!("{}", url);

                                    // get request to specific month and year
                                    let resp = reqwest::blocking::get(url).unwrap();
                                    assert!(resp.status().is_success());

                                    Document::from_read(resp)
                                        .unwrap()
                                        .find(Name("a"))
                                        .filter_map(|n| n.attr("href"))
                                        .for_each(|link|{
                                            if link.contains(chatlog) {
                                                // check if link is for the longs of a single day
                                                let mut flag = false;
                                                for bad_href in bad_hrefs.iter() {
                                                    if link.contains(bad_href) { flag = true };
                                                }
                                                if !flag {
                                                    let len = link.len();
                                                    let day = &link[len-2..];
                                                    let mut url: String = "https://overrustlelogs.net/Destinygg%20chatlog".to_owned();
                                                    let month_year: String = "/".to_owned() + *month + &*"%20".to_owned() + &*year.to_string();
                                                    let day_month_year: String = "/".to_owned() + &*year.to_string() + &"-".to_owned() + months_map[*month] + &"-".to_owned() + day + &".txt".to_string();

                                                    url.push_str(&month_year);
                                                    url.push_str(&day_month_year);

                                                    println!("{:?}", url);
                                                }
                                            }
                                        })
                                }
                            }
                        }
                    }
                }
        });

    // for link in links.iter_mut() {
    //     if link.contains("/Destinygg chatlog/") {
    //         for month in months.iter() {
    //             if link.contains(month) {
    //                 for year in &years {
    //                     if link.contains(year) {
    //                         println!("{:?}", link);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
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
