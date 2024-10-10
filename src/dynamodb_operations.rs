use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};
use aws_types::SdkConfig;
use std::collections::HashMap;
use colored::*;

// the function creates the dynamodb client using a passed sdk config
pub fn create_dynamodb_client(config: &SdkConfig) -> Client {
    Client::new(&config)
}

pub async fn verify_table_exists(client: &Client, table_name: &str) -> Result<(), Error> {
    match client.describe_table().table_name(table_name).send().await {
        Ok(_) => {
            println!("{}", format!("Table '{}' exists.", table_name).green());
            Ok(())
        },
        Err(_) => {
            eprintln!("{}", format!("Table '{}' does not exist.", table_name).red());
            std::process::exit(1);
        }
    }
}

pub async fn read_current_version(client: &Client, table_name: &str) -> Result<String, Error> {
    let service_key = "Service";
    let service_value = AttributeValue::S("Frontend".to_string());

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

    Ok(current_version)
}

pub async fn update_version(client: &Client, table_name: &str, version: &str) -> Result<(), Error> {
    let service_key = "Service";
    let service_value = AttributeValue::S("Frontend".to_string());

    let mut expression_attribute_values = HashMap::new();
    expression_attribute_values.insert(
        ":version".to_string(),
        AttributeValue::S(version.to_string()),
    );

    let mut update_item = client
        .update_item()
        .table_name(table_name)
        .key(service_key, service_value.clone())
        .update_expression("SET Version = :version");

    for (k, v) in expression_attribute_values {
        update_item = update_item.expression_attribute_values(k, v);
    }

    update_item.send().await?;

    Ok(())
}