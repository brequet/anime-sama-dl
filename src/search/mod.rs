use anyhow::{Context, Result};
use reqwest::blocking::Client;
use scraper::{Html, Selector};

/// A search result from anime-sama.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
}

/// Search anime-sama for shows matching the query.
pub fn search(client: &Client, query: &str) -> Result<Vec<SearchResult>> {
    let resp = client
        .post("https://anime-sama.to/template-php/defaut/fetch.php")
        .form(&[("query", query)])
        .send()
        .context("failed to search anime-sama")?;

    let html = resp.text()?;
    let doc = Html::parse_fragment(&html);
    let link_sel = Selector::parse("a.asn-search-result").unwrap();
    let title_sel = Selector::parse("h3.asn-search-result-title").unwrap();

    let results = doc
        .select(&link_sel)
        .filter_map(|el| {
            let url = el.value().attr("href")?.to_string();
            let title = el.select(&title_sel).next()?.text().collect::<String>();
            Some(SearchResult { title, url })
        })
        .collect();

    Ok(results)
}
