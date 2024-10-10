mod aws_config;
mod dynamodb_operations;
mod github_releases;
mod persistence;

use aws_config::create_aws_config;
use colored::*;
use dynamodb_operations::{
    create_dynamodb_client, read_current_version, update_version, verify_table_exists,
};
use github_releases::{list_github_releases, select_release, verify_release_exists};
use persistence::{get_config, reset_config, AppConfig};
use semver::Version;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check for the --reset flag
    if args.contains(&"--reset".to_string()) {
        reset_config()?;
        println!("{}", "Configuration has been reset.".green());
        return Ok(());
    }

    // Load the configuration
    let app_config = get_config()?;

    // Destructure the app_config
    let AppConfig {
        github_token,
        table_name,
        owner,
        repo,
    } = app_config;

    let releases = list_github_releases(github_token, owner, repo).await?;
    let config = create_aws_config().await?;
    let client = create_dynamodb_client(&config);

    // Verify if the table exists
    verify_table_exists(&client, &table_name).await?;

    // Read current version from DynamoDB
    let current_version = read_current_version(&client, &table_name).await?;

    println!();
    println!(
        "{}",
        format!("Current version: {}", current_version).yellow()
    );
    println!();

    // Determine the version to use
    let version = if args.len() > 1 {
        // Skip program name and check for other args
        let version_arg = args.iter().skip(1).find(|&arg| !arg.starts_with('-'));
        if let Some(version_value) = version_arg {
            // Verify the release exists
            if !verify_release_exists(&releases, version_value) {
                eprintln!("Error: The provided version does not exist.");
                std::process::exit(1);
            }
            version_value.clone()
        } else {
            // No version provided
            eprintln!("Error: No version provided.");
            std::process::exit(1);
        }
    } else {
        // Select a release using the select_release function
        let selected_release = select_release(releases, &current_version).await?;
        selected_release
    };

    // Validate version
    if Version::parse(&version).is_err() {
        println!();
        eprintln!(
            "{}",
            "Version is not a valid semver.\nExamples: 1.0.0, 2.1.3, 0.0.1, 1.0.0-alpha".red()
        );
        println!();
        std::process::exit(1);
    }

    // Compare versions
    if version == current_version {
        println!();
        println!(
            "{}",
            "The provided version is the same as the current version. No update needed.".yellow()
        );
        println!();
        return Ok(());
    }

    // Update version in DynamoDB
    update_version(&client, &table_name, &version).await?;

    println!();
    println!("{}", "Update successful!".green());
    println!();

    let new_version = read_current_version(&client, &table_name).await?;

    println!("{}", format!("New version: {}", new_version).green());
    println!();

    Ok(())
}