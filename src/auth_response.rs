//! This module defines the structures and error handling for authentication responses.
//!
//! It includes the `AuthResponse` struct for handling authentication responses
//! from the Salesforce API and the `AuthError` enum for capturing various types of errors
//! that may occur during the authentication process.

use serde::Deserialize;
use std::fmt;

/// Represents the response from the Salesforce authentication API.
///
/// This struct is used to deserialize the JSON response from the API
/// and contains the access token and instance URL needed for subsequent API requests.
#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    /// The access token used for authenticated requests.
    pub access_token: String,
    /// The instance URL for making API requests.
    pub instance_url: String,
}

/// Enum representing possible errors during the authentication process.
///
/// This enum captures different types of errors that can occur, such as request errors,
/// parsing errors, and custom errors.
#[derive(Debug)]
pub enum AuthError {
    /// Error occurring during the HTTP request.
    ReqwestError(reqwest::Error),
    /// Error occurring during the parsing of the response.
    ParseError(String),
    /// Custom error with a specific message.
    CustomError(String),
}

impl fmt::Display for AuthError {
    /// Formats the `AuthError` for displaying.
    ///
    /// This implementation provides a human-readable description of the error,
    /// depending on its variant.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::ReqwestError(e) => write!(f, "Request error: {}", e),
            AuthError::ParseError(e) => write!(f, "Parse error: {}", e),
            AuthError::CustomError(e) => write!(f, "Custom error: {}", e),
        }
    }
}

impl From<reqwest::Error> for AuthError {
    /// Converts a `reqwest::Error` into an `AuthError`.
    ///
    /// This implementation allows for automatic conversion of `reqwest::Error`
    /// into the `AuthError::ReqwestError` variant.
    fn from(error: reqwest::Error) -> Self {
        AuthError::ReqwestError(error)
    }
}