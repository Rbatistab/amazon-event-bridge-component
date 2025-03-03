//! PutEventsRequest module for Amazon EventBridge component.
//!
//! This module defines the structure and implementation for creating EventBridge
//! PutEvents requests. It handles the necessary parameters for sending events to
//! Amazon EventBridge:
//! ```
//! {
//!     "EndpointId": "string",
//!     "Entries": [
//!         {
//!             "Detail": "string",
//!             "DetailType": "string",
//!             "EventBusName": "string",
//!             "Resources": [ "string" ],
//!             "Source": "string",
//!             "Time": number,
//!             "TraceHeader": "string"
//!         }
//!     ]
//! }
//! ```

use anyhow::Context;
use std::collections::HashMap;
use std::str::FromStr;
use crate::apis::put_events_api::put_events_request_entry::PutEventsRequestEntry;
use crate::exports::edgee::components::data_collection::Dict;

/// A map of configuration settings with string keys and values
type StringConfigMap = HashMap<String, String>;
type Entries = Vec<PutEventsRequestEntry>;


/// Represents a request to the PutEvents API for Amazon EventBridge.
///
/// This structure contains all necessary parameters to make a PutEvents request,
/// including endpoint information, authentication details, and the event entries
/// to be published.
///
/// # Fields
/// - endpoint_id: The URL subdomain of the endpoint
/// - entries: Entries that defines an event in your system.
/// - access_key: (Optional?) TBD
/// - security_token: (Optional?) TBD
/// - iam_role: (Optional?) TBD
/// - access_policy: (Optional?) TBD
/// - vpc_endpoint: (Optional?) TBD
pub struct PutEventsRequest {
    endpoint_id: String,
    entries: Entries
    access_key: Option<String>,
    security_token: Option<String>,
    iam_role: Option<String>,
    access_policy: Option<String>,
    vpc_endpoint: Option<String>
}

impl PutEventsRequest {
    pub fn new(request_dict: Dict) -> anyhow::Result<Self> {
        let request_map: StringConfigMap = request_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let endpoint_id = request_map
            .get("aws_endpoint_id")
            .context("Missing AWS endpoint id")?
            .to_string();

        let entries_as_string = request_map
            .get("aws_entries")
            .context("Missing AWS entries that define event")?
            .to_string();

        // For now let's take only one entry
        let entry = PutEventsRequestEntry::from_str(&entries_as_string)?;
        let entries = vec![entry];

        // Optional fields
        let access_key = request_map.get("aws_access_key").cloned();
        let security_token = request_map.get("aws_security_token").cloned();
        let iam_role = request_map.get("aws_iam_role").cloned();
        let access_policy = request_map.get("aws_access_policy").cloned();
        let vpc_endpoint = request_map.get("aws_vpc_endpoint").cloned();

        Ok(Self {
            endpoint_id,
            entries,
            access_key,
            security_token,
            iam_role,
            access_policy,
            vpc_endpoint
        })
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