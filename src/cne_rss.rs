use scraper::{Html, Selector};
use std::error::Error;

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
