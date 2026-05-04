mod download;
mod episodes;
mod search;
mod vidmoly;

use anyhow::Result;
use dialoguer::{Input, Select};
use reqwest::blocking::Client;

fn main() -> Result<()> {
    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    // Step 1: Search for a show
    let query: String = Input::new()
        .with_prompt("Search anime")
        .interact_text()?;

    let results = search::search(&client, &query)?;
    if results.is_empty() {
        println!("No results found.");
        return Ok(());
    }

    let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
    let selection = Select::new()
        .with_prompt("Select a show")
        .items(&titles)
        .interact()?;

    let chosen = &results[selection];
    println!("Fetching episodes from: {}", chosen.url);

    // Step 2: Fetch vidmoly episode list
    let episodes = episodes::fetch_vidmoly_episodes(&client, &chosen.url)?;
    if episodes.is_empty() {
        println!("No vidmoly episodes found.");
        return Ok(());
    }

    let ep_labels: Vec<String> = (1..=episodes.len())
        .map(|i| format!("Episode {}", i))
        .collect();
    let ep_labels_str: Vec<&str> = ep_labels.iter().map(|s| s.as_str()).collect();

    let ep_sel = Select::new()
        .with_prompt("Select an episode")
        .items(&ep_labels_str)
        .interact()?;

    println!("Extracting stream URL...");
    let m3u8_url = vidmoly::extract_m3u8(&client, &episodes[ep_sel])?;
    println!("Found: {}", m3u8_url);

    // Step 3: Download
    let output = format!("{}-ep{:02}.mp4", chosen.title.replace(' ', "-"), ep_sel + 1);
    println!("Downloading to {}...", output);
    download::download_m3u8(&m3u8_url, &output)?;
    println!("Done!");

    Ok(())
}
