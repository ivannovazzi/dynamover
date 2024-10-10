// src/aws_config.rs
use aws_config::BehaviorVersion;
use aws_types::{SdkConfig, os_shim_internal::{Env, Fs}};
use aws_runtime::env_config;
use inquire::Select;

pub async fn create_aws_config() -> Result<SdkConfig, Box<dyn std::error::Error>> {
    let fs = Fs::real();
    let env = Env::real();
    let profile_files = env_config::file::EnvConfigFiles::default();
    let profiles_set = aws_config::profile::load(&fs, &env, &profile_files, None).await?;
    let section_names: Vec<String> = profiles_set
        .profiles()
        .map(|s| s.to_string())
        .collect();

    let aws_profile = Select::new("Select an AWS profile:", section_names)
        .prompt()?;

    // Set up AWS configuration loader
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .region("eu-west-1")
        .profile_name(&aws_profile)
        .load()
        .await;

    Ok(config)
}