use inquire::Text;
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};
use std::fs;
use dirs;
use std::path::Path;
use std::io::Write;


fn validate_token(token: &str) -> bool {
  // ## Personal access tokens (classic)
  // ^ghp_[a-zA-Z0-9]{36}$  
  // ## Fine-grained personal access tokens
  // ^github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59}$  
  // ^ghs_[a-zA-Z0-9]{36}$  
  // ## Combined together
  // ^(gh[ps]_[a-zA-Z0-9]{36}|github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59})$  
  let re = regex::Regex::new(r"^(gh[ps]_[a-zA-Z0-9]{36}|github_pat_[a-zA-Z0-9]{22}_[a-zA-Z0-9]{59})$").unwrap();
  re.is_match(token)
}

fn validate_repo(repo: &str) -> bool {
  // ^[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+$  
  let re = regex::Regex::new(r"^[a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+$").unwrap();
  re.is_match(repo)
}

fn validate_table_name(table_name: &str) -> bool {
  // ^[a-zA-Z0-9_.-]+$  
  let re = regex::Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
  re.is_match(table_name)
}

fn validate_owner(owner: &str) -> bool {
  // ^[a-zA-Z0-9_.-]+$  
  let re = regex::Regex::new(r"^[a-zA-Z0-9_.-]+$").unwrap();
  re.is_match(owner)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub github_token: String,
    pub table_name: String,
    pub owner: String,
    pub repo : String,
}

pub fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_file = ".dynamover.json";

    // configpath has to be user directory + .dynamover.json
    let path = format!("{}/{}", dirs::home_dir().unwrap().display(), config_file);
    let config_path = path.as_str();

    // Check if the config file exists
    if !Path::new(config_path).exists() {
        // Create a default AppConfig with empty github_token
        let app_config = AppConfig {
            github_token: String::new(),
            table_name: String::new(),
            owner: String::new(),
            repo: String::new(),
        };

        // Serialize the default AppConfig to JSON
        let json = serde_json::to_string_pretty(&app_config)?;

        // Write the JSON to the config file
        let mut file = fs::File::create(config_path)?;
        file.write_all(json.as_bytes())?;
    }

    // string to pointer config path

    let settings = Config::builder()
        .add_source(File::new(config_path, FileFormat::Json))        
        .build()?;
    
    // Deserialize the configuration into AppConfig
    let mut app_config: AppConfig = settings.try_deserialize()?;

    // If token is empty, prompt the user for the token and save it
    if app_config.github_token.is_empty() {
        let token = Text::new("Enter your GitHub Bearer Token:")
            .prompt()?;
        // Validate the token
        if !validate_token(&token) {
            eprintln!("Invalid token format. Please check your token and try again.");
            std::process::exit(1);
        }
        // Update the configuration
        app_config.github_token = token.clone();
    }

    if app_config.table_name.is_empty() {
        let table_name = Text::new("Enter your table name:")
            .prompt()?;

        // Validate the table name
        if !validate_table_name(&table_name) {
            eprintln!("Invalid table name format. Please check your table name and try again.");
            std::process::exit(1);
        }
        // Update the configuration
        app_config.table_name = table_name.clone();
    }

    if app_config.owner.is_empty() {
        let owner = Text::new("Enter your owner name:")
            .prompt()?;

        // Validate the owner name
        if !validate_owner(&owner) {
            eprintln!("Invalid owner name format. Please check your owner name and try again.");
            std::process::exit(1);
        }
        // Update the configuration
        app_config.owner = owner.clone();
    }

    if app_config.repo.is_empty() {
        let repo = Text::new("Enter your repo name:")
            .prompt()?;
        // Validate the repo name
        if !validate_repo(&repo) {
            eprintln!("Invalid repo name format. Please check your repo name and try again.");
            std::process::exit(1);
        }
        // Update the configuration
        app_config.repo = repo.clone();
    }



  // Serialize and save the updated configuration
  let json = serde_json::to_string_pretty(&app_config)?;
  let mut file = fs::File::create(config_path)?;
  file.write_all(json.as_bytes())?;

        

    Ok(app_config)
}