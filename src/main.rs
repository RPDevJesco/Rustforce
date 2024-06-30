mod auth_response;
mod constants;
mod salesforce_client;
mod salesforce_operations;

use std::collections::HashMap;
use serde_json::json;
use crate::constants::Constants;
use crate::salesforce_client::SalesforceClient;

/// Main function to run the Salesforce integration application.
///
/// Initializes the constants and Salesforce client, and attempts to authorize the client.
#[tokio::main]
async fn main() {
    let constants = Constants::new();
    let mut salesforce_client = SalesforceClient::new();

    if let Err(err) = salesforce_client.authorize(&constants).await {
        eprintln!("Authorization failed: {}", err);
        return;
    }

    // Example insert
    let mut data = HashMap::new();
    data.insert("Subject".to_string(), json!("Test case"));
    data.insert("Priority".to_string(), json!("High"));

    match salesforce_client.insert_record("Case", &data).await {
        Ok(id) => println!("Record created with ID: {}", id),
        Err(err) => eprintln!("Insert record failed: {}", err),
    }

    // Example query
    let query = "SELECT Id, Subject FROM Case LIMIT 10";
    match salesforce_client.query_records(query).await {
        Ok(result) => println!("Query result: {}", result),
        Err(err) => eprintln!("Query records failed: {}", err),
    }
}