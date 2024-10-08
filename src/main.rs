use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use aws_config::BehaviorVersion;
use semver::Version;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use colored::*;
use aws_runtime::env_config;
use inquire::Select;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let fs = aws_types::os_shim_internal::Fs::real();
    let env = aws_types::os_shim_internal::Env::real();
    let profile_files = env_config::file::EnvConfigFiles::default();
    let profiles_set = aws_config::profile::load(&fs, &env, &profile_files, None).await.unwrap();
    let section_names: Vec<String> = profiles_set
        .profiles()
        .map(|s| s.to_string())
        .collect();
    
    let aws_profile = Select::new("Select an AWS profile:", section_names)
        .prompt()
        .unwrap();

    // Set up AWS configuration loader
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region("eu-west-1")
        .profile_name(&aws_profile)
        .load()
        .await;

    let client = Client::new(&config);

    // Table name constant
    let table_name = "flare-platform-staging";
    // Prepare key
    let service_key = "Service";
    let service_value = AttributeValue::S("Frontend".to_string());

     // Read current version from DynamoDB
     let get_item_output = client
        .get_item()
        .table_name(table_name)
        .key(service_key, service_value.clone())
        .send()
        .await?;

    let current_version = if let Some(item) = get_item_output.item {
        item.get("Version")
            .and_then(|v| v.as_s().ok())
            .cloned()
            .unwrap_or_default()
    } else {
        "".to_string()
    };

    println!("{}", format!("Current version: {}", current_version).yellow());

    // Get version from command line arguments or prompt user
    let args: Vec<String> = env::args().collect();
    let version = if args.len() > 1 {
        args[1].clone()
    } else {
        // Prompt user for version
        print!("{}", "Please enter a new version: ".blue());
        io::stdout().flush().unwrap();
        let mut version_input = String::new();
        io::stdin().read_line(&mut version_input).unwrap();
        version_input.trim().to_string()
    };

    // Validate version
    if Version::parse(&version).is_err() {
      eprintln!(
          "{}",
          "Version is not a valid semver.\nExamples: 1.0.0, 2.1.3, 0.0.1, 1.0.0-alpha".red()
      );
      std::process::exit(1);
  }

    // Compare versions
    if version == current_version {
        println!(
            "{}",
            "The provided version is the same as the current version. No update needed.".yellow()
        );
        return Ok(());
    }

    // Prepare expression attribute values for update
    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":version".to_string(),
        AttributeValue::S(version.clone()),
    );

    // Update version in DynamoDB
    client
        .update_item()
        .table_name(table_name)
        .key(service_key, service_value.clone())
        .update_expression("SET Version = :version")
        .expression_attribute_values(":version", AttributeValue::S(version.clone()))
        .send()
        .await?;

    println!("{}", "Update successful!".green());
    println!("{}", format!("New version: {}", version).green());

    Ok(())
}