mod download;
mod episodes;
mod search;
mod vidmoly;

use anyhow::{bail, Result};
use clap::Parser;
use dialoguer::{Input, Select};
use reqwest::blocking::Client;

/// anime-sama-dl: download anime episodes from anime-sama.to
///
/// Usage examples:
///   anime-sama-dl                  # fully interactive
///   anime-sama-dl sorcier          # search "sorcier", pick episode interactively
///   anime-sama-dl sorcier -e 3     # download episode 3 directly
#[derive(Parser)]
#[command(name = "anime-sama-dl", version, about)]
struct Cli {
    /// Search query (anime name). If omitted, prompts interactively.
    query: Option<String>,

    /// Episode number to download. Skips episode selection if provided.
    #[arg(short, long)]
    episode: Option<usize>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    // Step 1: Get search query
    let query = match cli.query {
        Some(q) => q,
        None => Input::new().with_prompt("Search anime").interact_text()?,
    };

    let results = search::search(&client, &query)?;
    if results.is_empty() {
        bail!("No results found for '{}'.", query);
    }

    // Step 2: Pick show (skip if only one result)
    let chosen = if results.len() == 1 {
        println!("Found: {}", results[0].title);
        &results[0]
    } else {
        let titles: Vec<&str> = results.iter().map(|r| r.title.as_str()).collect();
        let selection = Select::new()
            .with_prompt("Select a show")
            .default(0)
            .items(&titles)
            .interact()?;
        &results[selection]
    };

    println!("Fetching episodes from: {}", chosen.url);

    // Step 3: Fetch vidmoly episode list
    let episodes = episodes::fetch_vidmoly_episodes(&client, &chosen.url)?;
    if episodes.is_empty() {
        bail!("No vidmoly episodes found.");
    }
    println!("Found {} episodes.", episodes.len());

    // Step 4: Pick episode
    let ep_idx = match cli.episode {
        Some(n) => {
            if n == 0 || n > episodes.len() {
                bail!("Episode {} out of range (1-{}).", n, episodes.len());
            }
            n - 1
        }
        None => {
            let ep_labels: Vec<String> = (1..=episodes.len())
                .map(|i| format!("Episode {}", i))
                .collect();
            let ep_labels_str: Vec<&str> = ep_labels.iter().map(|s| s.as_str()).collect();
            Select::new()
                .with_prompt("Select an episode")
                .default(0)
                .items(&ep_labels_str)
                .interact()?
        }
    };

    // Step 5: Extract m3u8 and download
    println!("Extracting stream URL...");
    let m3u8_url = vidmoly::extract_m3u8(&client, &episodes[ep_idx])?;
    println!("Stream: {}", m3u8_url);

    let output = format!("{}-ep{:02}.mp4", chosen.title.replace(' ', "-"), ep_idx + 1);
    println!("Downloading to {}...", output);
    download::download_m3u8(&m3u8_url, &output)?;
    println!("Done!");

    Ok(())
}
