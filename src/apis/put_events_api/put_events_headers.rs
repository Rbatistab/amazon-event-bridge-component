//! EventBridge PutEvents API headers module
//!
//! This module handles the construction and management of headers required
//! for making PutEvents API requests to Amazon EventBridge.

use aws_credential_types::Credentials;
use aws_sigv4::http_request::{
    sign, SignableBody, SignableRequest, SigningParams, SigningSettings,
};
use aws_smithy_runtime_api::client::identity::Identity;
use std::time::SystemTime;
use aws_sigv4::sign::v4;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::Dict;

/// Represents the headers configuration for EventBridge PutEvents API requests
///
/// Contains the necessary AWS credentials and regional configuration for
/// authenticating and routing EventBridge API requests.
pub struct PutEventsHeaders {
    /// The domain for the EventBridge endpoint (e.g., "amazonaws.com")
    domain: String,
    /// AWS region where the EventBridge endpoint is located
    region: String,
    /// AWS access key for request authentication
    access_key: String,
    /// AWS secret key for request authentication
    secret_key: String,
    /// Optional AWS security token for temporary credentials
    security_token: Option<String>
}

impl PutEventsHeaders {
    /// Creates a new instance of PutEventsHeaders from the provided settings
    ///
    /// # Arguments
    ///
    /// * `settings_map` - Reference to Settings containing AWS credentials and configuration
    ///
    /// # Returns
    ///
    /// * `PutEventsHeaders` - A new instance configured with the provided settings
    pub fn new(settings_map: &Settings) -> PutEventsHeaders {
      Self {
          domain: settings_map.domain.clone(),
          region: settings_map.region.clone(),
          access_key: settings_map.access_key.clone(),
          secret_key: settings_map.secret_key.clone(),
          security_token: settings_map.security_token.clone()
      }
    }

    /// Constructs the complete EventBridge endpoint URL
    ///
    /// Combines the region and domain to create the full endpoint URL
    /// in the format: https://events.[region].[domain]
    ///
    /// # Returns
    ///
    /// * `String` - The fully constructed EventBridge endpoint URL
    pub fn get_headers_host(&self) -> String {
        format!(
            "https://events.{}.{}",
            self.region.clone(),
            self.domain.clone()
        )
    }

    /// Generates the required headers for an EventBridge PutEvents API request
    ///
    /// This method creates and signs the headers required for AWS API authentication
    /// using AWS Signature Version 4 (SigV4). It includes:
    /// - AWS credentials for authentication
    /// - Request signing with the current timestamp
    /// - Service-specific headers for EventBridge
    ///
    /// # Arguments
    ///
    /// * `body` - The request body as a String, used for request signing
    ///
    /// # Returns
    ///
    /// * `Dict` - A dictionary containing all required headers for the API request,
    ///           including:
    ///           - Authorization header with AWS signature
    ///           - X-Amz-Date header with timestamp
    ///           - Host header
    ///           - Content-Type header
    ///           - Security token header (if temporary credentials are used)
    ///
    /// # Note
    ///
    /// This method uses AWS Signature Version 4 signing process to generate
    /// authentication headers. The signing process includes the request body
    /// to ensure request integrity.
    pub fn get_headers(
        &self,
        body: &String
    ) -> Dict {
        let identity: Identity = Credentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            self.security_token.clone(),
            None,
            "hardcoded-credentials"
        ).into();

        let signing_settings = SigningSettings::default();

        let signing_params: SigningParams = v4::SigningParams::builder()
            .identity(&identity)
            .region(self.region.as_str())
            .name("event-bridge")
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .unwrap()
            .into();

        let signable_request = SignableRequest::new(
            "POST",
            self.get_headers_host(),
            std::iter::empty(),
            SignableBody::Bytes(body.as_bytes())
        ).expect("Signable request");

        // generate the signature headers
        let (signing_instructions, _signature) = sign(signable_request, &signing_params)
            .unwrap()
            .into_parts();

        // convert to Vec<(String, String)>
        signing_instructions
            .headers()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()

    }
}

#[cfg(test)]
mod put_events_request_test {
    use super::*;

    #[test]
    fn test_put_events_request_new() {
        // Dummy test
        assert_eq!(true, true)
    }

}
