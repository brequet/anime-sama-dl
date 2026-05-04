use anyhow::{bail, Context, Result};
use regex::Regex;
use reqwest::blocking::Client;

pub fn extract_m3u8(client: &Client, embed_url: &str) -> Result<String> {
    let body = client
        .get(embed_url)
        .send()
        .context("failed to fetch vidmoly page")?
        .text()?;
    parse_m3u8_from_html(&body)
}

fn parse_m3u8_from_html(html: &str) -> Result<String> {
    let re = Regex::new(r"https?://[^\s']+\.m3u8[^\s']*").unwrap();
    match re.find(html) {
        Some(m) => Ok(m.as_str().to_string()),
        None => bail!("no m3u8 URL found in vidmoly page"),
    }
}
