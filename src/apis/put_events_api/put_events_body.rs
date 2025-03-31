//! PutEventsBody module for Amazon EventBridge component.
//!
//! This module defines the structure and implementation for creating EventBridge
//! PutEvents requests. It handles the necessary parameters for sending events to
//! Amazon EventBridge:
//! ```
//! {
//!     "EndpointId": "https://abcde.veo.endpoints.event.amazonaws.com",
//!     "Entries": [
//!         {
//!             "Detail": "{ \"key1\": \"value1\", \"key2\": \"value2\" }",
//!             "DetailType": "myDetailType",
//!             "EventBusName": "myEventBusName",
//!             "Resources": [ "string" ],
//!             "Source": "com.mycompany.myapp",
//!             "Time": number,
//!             "TraceHeader": "myTraceHeader"
//!         }
//!     ]
//! }
//! ```

use crate::apis::put_events_api::put_events_body_entry::PutEventsRequestEntry;
use crate::exports::edgee::components::data_collection::Event;

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
///
/// # Note
/// Dummy values for now, since we don't know if we would need a body representation as part
/// of the event
#[allow(dead_code)]
pub struct PutEventsBody {
    endpoint_id: String,
    entries: Entries,
}

impl PutEventsBody {
    /// Creates a new instance of PutEventsBody with default values
    ///
    /// This method initializes a PutEventsBody with empty values for testing
    /// and development purposes. The endpoint_id is set to an empty string
    /// and entries are initialized as empty.
    ///
    /// # Returns
    ///
    /// * `Result<PutEventsBody, anyhow::Error>` - A new PutEventsBody instance wrapped in Result
    ///
    /// # Note
    ///
    /// Currently uses dummy values as the body representation requirements
    /// for the event are not yet finalized.
    #[allow(dead_code)]
    pub fn new() -> anyhow::Result<Self> {
        // Dummy values for now, since we don't know if we would need a body representation as part
        // of the event
        Ok(Self {
            endpoint_id: String::new(),
            entries: Entries::new(),
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
