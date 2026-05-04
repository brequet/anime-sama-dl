use anyhow::{bail, Context, Result};
use std::process::Command;

/// Download an m3u8 stream to an mp4 file using ffmpeg.
pub fn download_m3u8(m3u8_url: &str, output_path: &str) -> Result<()> {
    let status = Command::new("ffmpeg")
        .args(["-i", m3u8_url, "-c", "copy", output_path])
        .status()
        .context("failed to run ffmpeg — is it installed?")?;

    if !status.success() {
        bail!("ffmpeg exited with status {}", status);
    }
    Ok(())
}
