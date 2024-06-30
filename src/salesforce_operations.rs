//! This module defines operations for inserting data into Salesforce and reading data from Salesforce.
//!
//! It includes methods for creating new records and querying existing records using the `SalesforceClient`.

use crate::constants::Constants;
use crate::auth_response::AuthError;
use reqwest::{Client, StatusCode};
use serde_json::Value;
use std::collections::HashMap;
use crate::salesforce_client::SalesforceClient;

impl SalesforceClient {
    /// Inserts a new record into Salesforce.
    ///
    /// This method sends a POST request to the Salesforce API to create a new record
    /// with the provided data.
    ///
    /// # Arguments
    ///
    /// * `object_type` - The type of the Salesforce object (e.g., "Case").
    /// * `data` - A reference to a `HashMap` containing the fields and values for the new record.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The ID of the created record if the insertion is successful.
    /// * `Err(AuthError)` - If an error occurs during the insertion process.
    pub async fn insert_record(
        &self,
        object_type: &str,
        data: &HashMap<String, Value>,
    ) -> Result<String, AuthError> {
        let client = Client::new();
        let request_url = format!(
            "{}/services/data/v60.0/sobjects/{}",
            self.instance_url.as_ref().unwrap(),
            object_type
        );

        let res = client
            .post(&request_url)
            .bearer_auth(self.token.as_ref().unwrap())
            .json(data)
            .send()
            .await?;

        let status = res.status();
        let response_text = res.text().await.unwrap_or_else(|_| "Failed to read response".to_string());

        if status != StatusCode::CREATED {
            return Err(AuthError::CustomError(format!(
                "Failed to create record. Status: {} - {}",
                status, response_text
            )));
        }

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| AuthError::ParseError(e.to_string()))?;
        let id = response_json["id"]
            .as_str()
            .ok_or_else(|| AuthError::ParseError("Missing ID in response".to_string()))?;

        Ok(id.to_string())
    }

    /// Queries records from Salesforce.
    ///
    /// This method sends a GET request to the Salesforce API to query records
    /// based on the provided SOQL query.
    ///
    /// # Arguments
    ///
    /// * `query` - The SOQL query string to be executed.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The query result as a `serde_json::Value` if the query is successful.
    /// * `Err(AuthError)` - If an error occurs during the query process.
    pub async fn query_records(&self, query: &str) -> Result<Value, AuthError> {
        let client = Client::new();
        let request_url = format!(
            "{}/services/data/v60.0/query?q={}",
            self.instance_url.as_ref().unwrap(),
            query
        );

        let res = client
            .get(&request_url)
            .bearer_auth(self.token.as_ref().unwrap())
            .send()
            .await?;

        let status = res.status();
        let response_text = res.text().await.unwrap_or_else(|_| "Failed to read response".to_string());

        if status != StatusCode::OK {
            return Err(AuthError::CustomError(format!(
                "Failed to query records. Status: {} - {}",
                status, response_text
            )));
        }

        let response_json: Value = serde_json::from_str(&response_text)
            .map_err(|e| AuthError::ParseError(e.to_string()))?;

        Ok(response_json)
    }
}