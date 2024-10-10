use inquire::Text;
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub github_token: String,
    pub table_name: String,
    pub owner: String,
    pub repo: String,
}

const CONFIG_FILE: &str = ".dynamover.json";

fn get_config_path() -> String {
  format!("{}/{}", dirs::home_dir().unwrap().display(), CONFIG_FILE)
}

pub fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    
    // Check if the config file exists
    if !Path::new(&get_config_path()).exists() {
        // Create a default AppConfig with empty fields
        let app_config = AppConfig {
            github_token: String::new(),
            table_name: String::new(),
            owner: String::new(),
            repo: String::new(),
        };

        // Serialize the default AppConfig to JSON
        let json = serde_json::to_string_pretty(&app_config)?;

        // Write the JSON to the config file
        let mut file = fs::File::create(&get_config_path())?;
        file.write_all(json.as_bytes())?;
    }

    // Load the configuration
    let settings = Config::builder()
        .add_source(File::new(&get_config_path(), FileFormat::Json))
        .build()?;

    // Deserialize the configuration into AppConfig
    let mut app_config: AppConfig = settings.try_deserialize()?;

    // Check and prompt for each field if necessary
    if app_config.github_token.is_empty() {
        let token = Text::new("Enter your GitHub Bearer Token:")
            .prompt()?;
        app_config.github_token = token;
        save_config(&app_config)?;
    }

    if app_config.table_name.is_empty() {
        let table_name = Text::new("Enter your DynamoDB table name:")
            .prompt()?;
        app_config.table_name = table_name;
        save_config(&app_config)?;
    }

    if app_config.owner.is_empty() {
        let owner = Text::new("Enter the GitHub repository owner:")
            .prompt()?;
        app_config.owner = owner;
        save_config(&app_config)?;
    }

    if app_config.repo.is_empty() {
        let repo = Text::new("Enter the GitHub repository name:")
            .prompt()?;
        app_config.repo = repo;
        save_config(&app_config)?;
    }

    Ok(app_config)
}

pub fn save_config(app_config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(&app_config)?;
    let mut file = fs::File::create(&get_config_path())?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn reset_config() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(&get_config_path()).exists() {
        fs::remove_file(&get_config_path())?;
    }
    println!("Configuration file '{}' has been deleted.", &get_config_path());
    Ok(())
}