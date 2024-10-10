// src/github_releases.rs

use serde::Deserialize;
use colored::*;
use inquire::Select;
use chrono::{DateTime, Utc};



#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    pub published_at: Option<String>,
}

#[derive(Clone)]
struct ReleaseOption {
    tag_name: String,
    display_name: String,
    is_latest: bool,
}

impl std::fmt::Display for ReleaseOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_latest {
            write!(f, "{}", self.display_name.bright_green())
        } else {
            write!(f, "{}", self.display_name)
        }
    }
}

pub async fn list_github_releases(token: String, owner: String, repo: String) -> Result<Vec<Release>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        owner, repo
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "flare-rust-app")
        // append the token to the header using the bearer format
        .header("Authorization", format!("Bearer {}", token))
        
        .send()
        .await?;

    if response.status().is_success() {
        let releases = response.json::<Vec<Release>>().await?;
        Ok(releases)
    } else if response.status().as_u16() == 401 {
        eprintln!("Error: Unauthorized access. Please check your authentication.");
        Ok(vec![])
    } else if response.status().as_u16() == 403 {
        eprintln!("Error: Forbidden access. You might have exceeded the API rate limit or lack permissions.");
        Ok(vec![])
    } else {
        eprintln!(
            "Error: Failed to fetch releases. Status code: {}",
            response.status()
        );
        Ok(vec![])
    }
}

pub async fn select_release(
    releases: Vec<Release>,
    latest_tag: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut release_options: Vec<ReleaseOption> = Vec::new();

    for release in releases.iter() {
        let tag_name = release.tag_name.trim_start_matches('v').to_string();
        let is_latest = tag_name == latest_tag;

        // Format the published_at date
        let published_date = if let Some(published_at) = &release.published_at {
            let parsed_date = DateTime::parse_from_rfc3339(published_at)?;
            let formatted_date = parsed_date.with_timezone(&Utc).format("%Y-%m-%d").to_string();
            formatted_date
        } else {
            "Unknown date".to_string()
        };

        // Use other fields in the display name
        let display_name = format!(
            "{} - {} ({})",
            tag_name,
            release.name.clone().unwrap_or_default(),
            published_date
        );

        release_options.push(ReleaseOption {
            tag_name,
            display_name,
            is_latest,
        });
    }

    let selected_option = Select::new("Select a release:", release_options)
        .prompt()?;

    Ok(selected_option.tag_name)
}

// Function to verify if a release exists
pub fn verify_release_exists(
    releases: &Vec<Release>,
    version: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let version = format!("v{}", version);
    if releases.iter().any(|r| r.tag_name == version) {
        Ok(())
    } else {
        eprintln!("Error: Release version '{}' does not exist.", version);
        std::process::exit(1);
    }
}