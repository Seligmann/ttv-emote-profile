extern crate reqwest;
extern crate scraper;

// use scraper::Html;
// use soup::prelude::*;
// use scraper::{Html, Selector};
use reqwest::StatusCode;

mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let url = "https://overrustlelogs.net/stalk?channel=Destinygg&nick=Ze1ig";
    let result = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("LOOOOLL BAD BAD BAD"),
    };

    println!("HTML: {}", raw_html);
}

// fn des_tiny(url: &str) {
    // for item in li_tags {
    //     let item_text = item.text().collect::<Vec<_>>();
    //     println!("{:?}", item_text);
    // }

    // let resp = reqwest::blocking::get(url).unwrap();
    // assert!(resp.status().is_success());
    //
    // let body = resp.text().unwrap();
    //
    // let soup = Soup::new(&body);
    // let div = soup.tag("div")
    //     .find()
    //     .expect("Couldn't find tag 'div'");
    // let mut span = div.children().filter(|child| child.is_element());
    // println!("{:?}", span.next().map(|tag| tag.next().to_string()));

    // let fragment = Html::parse_document(&body);
    // let memes = Selector::parse(".text").unwrap();
    //
    // for element in fragment.select(&memes) {
    //     let element_txt = element.text().collect::<Vec<_>>();
    //     println!("{:?}", element_txt[0]);
    // }
// }

