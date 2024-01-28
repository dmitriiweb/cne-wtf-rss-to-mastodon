use log::error;
use scraper::{Html, Selector};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;

use crate::config::Config;
use reqwest;
use rss::Channel;

#[derive(Debug)]
pub struct RssFeed {
    pub title: String,
    pub link: String,
    pub description: String,
}

impl RssFeed {
    pub fn new(config: &Config) -> Result<Vec<Self>, Box<dyn Error>> {
        let content = reqwest::blocking::get(config.rss_url.clone())?.bytes()?;
        let channel = Channel::read_from(&content[..])?;
        let mut feeds: Vec<RssFeed> = vec![];
        for item in channel.items() {
            let title = match &item.title {
                Some(title) => title.clone(),
                None => continue,
            };
            let link = match &item.link {
                Some(link) => link.clone(),
                None => continue,
            };
            let description = match &item.description {
                Some(description) => RssFeed::remove_html(description.clone()),
                None => String::from("N/A"),
            };
            feeds.push(RssFeed {
                title,
                link,
                description,
            });
        }
        Ok(feeds)
    }

    fn remove_html(html_text: String) -> String {
        let fragment = Html::parse_fragment(&html_text);
        let selector = Selector::parse("p").unwrap();
        let p = fragment.select(&selector).next().unwrap();
        let texts = p.text().collect::<Vec<_>>();
        let text = texts[0].to_string();
        format!("{}...", text)
    }
}

impl RssFeed {
    pub fn filter_by_url(feeds: Vec<RssFeed>, config: &Config) -> Vec<RssFeed> {
        let file_path = Path::new(&config.saved_urls_file);
        if !file_path.exists() {
            RssFeed::save_urls_to_file(&config.saved_urls_file, &feeds);
            feeds
        } else {
            let saved_urls = RssFeed::get_urls(&config.saved_urls_file);
            let mut result: Vec<RssFeed> = vec![];
            for i in feeds.into_iter() {
                if !saved_urls.contains(&i.link) {
                    result.push(i);
                }
            }
            RssFeed::save_urls_to_file(&config.saved_urls_file, &result);
            result
        }
    }

    fn save_urls_to_file(file_path: &str, feeds: &[RssFeed]) {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap_or_else(|err| {
                error!("Can't open file with urls: {}", err);
                std::process::exit(1);
            });
        for i in feeds.iter() {
            let line = format!("{}\n", &i.link);
            if let Err(err) = write!(file, "{}", line) {
                error!("Can't write to file with urls: {}", err);
                std::process::exit(1);
            };
        }
    }

    fn get_urls(file_path: &str) -> Vec<String> {
        let file = File::open(file_path).unwrap();
        let buf = BufReader::new(file);
        let mut result: Vec<String> = vec![];
        for line in buf.lines() {
            let clean_url = line.unwrap().trim().to_string();
            result.push(clean_url);
        }
        result
    }
}
