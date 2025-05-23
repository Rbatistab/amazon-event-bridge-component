use crate::exports::edgee::components::data_collection::Dict;
use anyhow::Context;
use std::collections::HashMap;

/// A map of configuration settings with string keys and values
type StringConfigMap = HashMap<String, String>;

/// Represents Edgee component settings
///
/// This structure contains all necessary parameters to make a PutEvents request,
/// including endpoint information, authentication details, and the event entries
/// to be published.
///
/// # Fields
/// - `api_variant`: Variant of EventBridge APIs
/// - `domain`: (Optional) AWS domain, defaults to 'amazonaws.com' if not provided
/// - `region`: AWS region
/// - `access_key`: AWS access key
/// - `secret_key`: AWS secret key to create the signature
/// - `security_token`: (Optional) Session security token
#[derive(Debug)]
pub struct Settings {
    /// The AWS service domain (defaults to "amazonaws.com")
    pub domain: String,
    /// AWS region where the EventBridge service will be accessed
    pub region: String,
    /// AWS access key ID for authentication
    pub access_key: String,
    /// AWS secret access key for authentication
    pub secret_key: String,
    /// Optional AWS security token for temporary credentials
    pub security_token: Option<String>,
}

impl Settings {
    /// Creates a new Settings instance from a map of configuration values.
    ///
    /// # Arguments
    ///
    /// * `request_map` - A HashMap containing the configuration key-value pairs
    ///
    /// # Returns
    ///
    /// * `Result<Settings>` - A new Settings instance if successful, or an error if required fields are missing
    ///
    /// # Required Keys in request_map
    ///
    /// * "api" - The API variant to use
    /// * "region" - AWS region
    /// * "aws_access_key" - AWS access key ID
    /// * "aws_secret_key" - AWS secret access key
    ///
    /// # Optional Keys
    ///
    /// * "domain" - Custom domain (defaults to "amazonaws.com")
    /// * "aws_security_token" - AWS security token for temporary credentials
    pub fn new(settings_dict: &Dict) -> anyhow::Result<Self> {
        let request_map: StringConfigMap = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let region = request_map
            .get("aws_region")
            .context("Missing AWS region")?
            .to_string();

        let domain = request_map
            .get("domain")
            .map(|s| s.to_string())
            .unwrap_or_else(|| "amazonaws.com".to_string());

        let access_key = request_map
            .get("aws_access_key")
            .context("Missing AWS access key")?
            .to_string();

        let secret_key = request_map
            .get("aws_secret_key")
            .context("Missing AWS secret key")?
            .to_string();

        let security_token = request_map.get("aws_security_token").cloned();

        Ok(Self {
            domain,
            region,
            access_key,
            secret_key,
            security_token,
        })
    }
}

#[cfg(test)]
mod settings_tests {
    use super::*;

    #[test]
    fn test_settings() {
        // TBD
        assert_eq!(true, true);
    }
}
