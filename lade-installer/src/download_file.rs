use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::{
    error::Error,
    fs,
    io::{Read, Write},
    path::PathBuf,
};

use crate::{download_urls::ZIP_NAME, info, paths::lade_cache_path};

pub fn download_file(url: &str) -> Result<PathBuf, Box<dyn Error>> {
    let client = Client::new();

    let mut response = client.get(url).send()?;

    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }

    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40}] {bytes}/{total_bytes} ({eta})",
        )?
        .progress_chars("#>-"),
    );

    let out = lade_cache_path().join(ZIP_NAME);

    if !out.exists() {
        fs::create_dir_all(lade_cache_path())?;
    }

    let mut file = fs::File::create(out)?;
    let mut buffer = [0; 8192];
    let mut downloaded = 0;

    while let Ok(len) = response.read(&mut buffer) {
        if len == 0 {
            break;
        }
        file.write_all(&buffer[..len])?;
        downloaded += len as u64;
        pb.set_position(downloaded);
    }

    pb.finish_and_clear();
    info!("Downloaded {}", ZIP_NAME);

    let out = lade_cache_path().join(ZIP_NAME);

    Ok(out)
}
