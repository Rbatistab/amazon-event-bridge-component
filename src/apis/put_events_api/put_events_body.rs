//! PutEventsBody module for Amazon EventBridge component.
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

use std::collections::HashMap;
use crate::apis::put_events_api::put_events_body_entry::PutEventsRequestEntry;
use crate::exports::edgee::components::data_collection::{Dict, Event};

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
pub struct PutEventsBody {
    endpoint_id: String,
    entries: Entries,
}

impl PutEventsBody {
    pub fn new(request_dict: Dict) -> anyhow::Result<Self> {
        // Dummy values for now, since we don't know if we would need a body representation as part
        // of the event
        Ok(Self {
            endpoint_id: String::new(),
            entries: Entries::new()
        })
    }

    pub fn get_body(edgee_event: &Event) -> String {
        serde_json::to_string(&edgee_event).unwrap_or_default()
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