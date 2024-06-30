//! This module defines the `SalesforceClient` struct and its associated methods for
//! interacting with the Salesforce API.
//!
//! The `SalesforceClient` struct handles the authentication process with Salesforce
//! and manages the access token and instance URL needed for making authenticated requests.

use crate::constants::Constants;
use crate::auth_response::{AuthResponse, AuthError};
use reqwest::{Client, StatusCode};

/// Represents a client for interacting with the Salesforce API.
///
/// This struct is responsible for managing the authentication process and
/// storing the access token and instance URL needed for making API requests.
pub struct SalesforceClient {
    /// The access token used for authenticated requests.
    pub(crate) token: Option<String>,
    /// The instance URL for making API requests.
    pub(crate) instance_url: Option<String>,
}

impl SalesforceClient {
    /// Creates a new `SalesforceClient` instance.
    ///
    /// This method initializes the `SalesforceClient` with empty token and instance URL fields.
    pub fn new() -> Self {
        SalesforceClient {
            token: None,
            instance_url: None,
        }
    }

    /// Authenticates the client with Salesforce using the provided constants.
    ///
    /// Sends an authorization request to the Salesforce API and stores the access token
    /// and instance URL if the authentication is successful.
    ///
    /// # Arguments
    ///
    /// * `constants` - A reference to the `Constants` struct containing the necessary credentials and endpoint information.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the authentication is successful.
    /// * `Err(AuthError)` - If an error occurs during the authentication process.
    pub async fn authorize(&mut self, constants: &Constants) -> Result<(), AuthError> {
        let client = Client::new();
        let params = [
            ("grant_type", "password"),
            ("client_id", &constants.consumer_key),
            ("client_secret", &constants.consumer_secret),
            ("username", &constants.username),
            ("password", &format!("{}{}", constants.password, constants.token)),
        ];

        let res = client
            .post(&constants.token_request_endpoint_url())
            .form(&params)
            .send()
            .await?;

        let status = res.status();
        let error_text = res.text().await.unwrap_or_else(|_| "Failed to read response".to_string());

        if status != StatusCode::OK {
            return Err(AuthError::CustomError(format!("Error: {} - {}", status, error_text)));
        }

        let auth_response: AuthResponse = serde_json::from_str(&error_text)
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        self.token = Some(auth_response.access_token);
        self.instance_url = Some(auth_response.instance_url);

        println!("Instance URL: {}", self.instance_url.as_ref().unwrap());
        println!("Access Token: {}", self.token.as_ref().unwrap());

        Ok(())
    }
}