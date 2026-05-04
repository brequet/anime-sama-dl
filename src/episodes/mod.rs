use anyhow::{Context, Result};
use regex::Regex;
use reqwest::blocking::Client;

pub fn fetch_vidmoly_episodes(client: &Client, catalogue_url: &str) -> Result<Vec<String>> {
    let episodes_url = format!(
        "{}/saison1/vostfr/episodes.js",
        catalogue_url.trim_end_matches('/')
    );
    let body = client
        .get(&episodes_url)
        .send()
        .context("failed to fetch episodes.js")?
        .text()?;
    extract_vidmoly_urls(&body)
}

pub fn extract_vidmoly_urls(js_content: &str) -> Result<Vec<String>> {
    let re = Regex::new(r"https://vidmoly\.to/embed-[\w.]+").unwrap();
    let urls: Vec<String> = re
        .find_iter(js_content)
        .map(|m| m.as_str().replace("vidmoly.to", "vidmoly.biz"))
        .collect();
    Ok(urls)
}
