extern crate reqwest;
extern crate scraper;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

fn main() {
    download("https://overrustlelogs.net/Destinygg%20chatlog/February%202022/2022-02-22.txt",
        "2022-02-22.txt");
    download("https://cdn.destiny.gg/emotes/emotes.json", "emotes.txt");

    // read the file and print each line
    if let Ok(lines) = read_lines("./emotes.txt") {
        for line in lines {
            if let Ok(messageInfo) = line {
                println!("{}", messageInfo);
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