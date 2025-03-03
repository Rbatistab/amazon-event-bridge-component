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


pub struct PutEventsRequest {
    /// The URL subdomain of the endpoint
    endpoint_id: String,
    /// Entries that defines an event in your system.
    entries: Entries
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

        let entries = "".to_string();
        Ok(Self {
            endpoint_id,
            entries: vec![entry]
        })
    }

}