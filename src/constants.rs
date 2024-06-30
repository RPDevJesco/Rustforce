//! This module defines the `Constants` struct and its associated methods for managing
//! configuration values needed for interacting with the Salesforce API.
//!
//! The `Constants` struct reads configuration from an INI file and provides methods
//! for accessing and constructing relevant API endpoint URLs.

use ini::Ini;
use std::fs::File;
use std::io::Write;

/// Represents configuration constants for the Salesforce integration.
///
/// This struct is used to store and manage the various keys, tokens, and endpoints
/// required to authenticate and interact with the Salesforce API.
pub struct Constants {
    /// The consumer key of the Salesforce application.
    pub consumer_key: String,
    /// The consumer secret of the Salesforce application.
    pub consumer_secret: String,
    /// The username of the Salesforce account.
    pub username: String,
    /// The token of the Salesforce account.
    pub token: String,
    /// The password of the Salesforce account.
    pub password: String,
    /// The endpoint URL of the Salesforce instance.
    pub endpoint: String,
}

impl Constants {
    /// Creates a new `Constants` instance and initializes it from the configuration file.
    ///
    /// If the configuration file does not exist, it creates a new one with default values.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to create or write to the configuration file,
    /// or if it fails to read from the configuration file.
    pub fn new() -> Self {
        let ini_file_path = "salesforce_config.ini";

        // Only write the configuration to the INI file if it doesn't exist
        if !std::path::Path::new(ini_file_path).exists() {
            let mut file = File::create(ini_file_path).expect("Could not create INI file");
            file.write_all(
                b"[Salesforce]\nSalesforceVersionNumber=60.0\nCONSUMER_KEY=ConsumerKey\nCONSUMER_SECRET=ConsumerSecret\nUSERNAME=LoginUsername\nTOKEN=SecurityToken\nENDPOINT=salesforceinstanceurl\n",
            )
                .expect("Could not write to INI file");

            println!("INI file '{}' has been created!", ini_file_path);
        }

        let conf = Ini::load_from_file(ini_file_path).expect("Could not read INI file");

        let section = conf.section(Some("Salesforce")).expect("Section [Salesforce] missing");
        Constants {
            consumer_key: section["CONSUMER_KEY"].to_string(),
            consumer_secret: section["CONSUMER_SECRET"].to_string(),
            username: section["USERNAME"].to_string(),
            token: section["TOKEN"].to_string(),
            password: "GDMERPDevStud10$!@".to_string(),
            endpoint: section["ENDPOINT"].to_string(),
        }
    }

    /// Returns the endpoint URL for requesting a token.
    ///
    /// This method constructs the full URL needed to obtain an OAuth2 token
    /// from the Salesforce API based on the configured endpoint.
    ///
    /// # Returns
    ///
    /// A `String` containing the full URL for the token request.
    pub fn token_request_endpoint_url(&self) -> String {
        format!("{}/services/oauth2/token", self.endpoint)
    }
}