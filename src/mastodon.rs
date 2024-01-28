use reqwest;
use reqwest::header::HeaderMap;

use crate::{cne_rss::RssFeed, config::Config};

#[derive(Debug)]
pub struct MastodonApi {
    token: String,
    pub base_url: String,
    pub max_post_len: i32,
}

impl MastodonApi {
    pub fn new(config: &Config) -> Self {
        Self {
            token: config.mastodon_token.clone(),
            base_url: config.mastodon_url.clone(),
            max_post_len: config.max_post_len,
        }
    }
}

impl MastodonApi {
    pub fn publish_posts(&self, feeds: &[RssFeed]) {
        let client = reqwest::blocking::Client::new();
        let url = self.url();
        for i in feeds.iter() {
            let headers = self.headers();
            let post = self.generate_post(i);
            let json_body = self.json_body(&post);
            let _ = client
                .post(&url)
                .headers(headers)
                .multipart(json_body)
                .send();
        }
    }

    fn generate_post(&self, feed: &RssFeed) -> String {
        format!(
            "{}\n\n\
            {}\n\n\
            {}\n\
            #Cabodia #news",
            feed.title, feed.description, feed.link
        )
    }

    fn url(&self) -> String {
        self.base_url.clone() + "/api/v1/statuses"
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let auth_key = format!("Bearer {}", &self.token);
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(&auth_key).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("multipart/form-data"),
        );
        headers
    }

    fn json_body(&self, post: &str) -> reqwest::blocking::multipart::Form {
        let status = reqwest::blocking::multipart::Part::text(post.to_string());
        let visibility = reqwest::blocking::multipart::Part::text("public".to_string());
        reqwest::blocking::multipart::Form::new()
            .part("status", status)
            .part("visibility", visibility)
    }
}
