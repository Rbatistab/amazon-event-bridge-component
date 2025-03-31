/// Represents an entry in a PutEvents request for Amazon EventBridge.
///
/// This struct encapsulates all the necessary fields required to publish an event
/// to Amazon EventBridge. It implements serialization and deserialization via serde,
/// and provides a conversion from String to support parsing JSON representations.
use chrono::offset::Utc;
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents an entry in a PutEvents request for Amazon EventBridge.
///
/// This struct contains all the information needed to publish an event to Amazon EventBridge.
/// Each entry represents a single event that will be delivered to the specified event bus.
///
/// # Fields
///
/// * `detail` - A valid JSON object.
/// * `detail_type` - Free-form string to decide what fields to expect in the event detail.
/// * `event_bus_name` - (Optional) The name or ARN of the event bus to receive the event.
/// * `source` - The source of the event.
/// * `resources` - (Optional) AWS resources, identified by Amazon Resource Name (ARN).
/// * `time` - (Optional) The time stamp of the event.
/// * `trace_header` - (Optional) An AWS X-Ray trace header.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PutEventsRequestEntry {
    #[serde(rename = "Detail")]
    detail: String,
    #[serde(rename = "DetailType")]
    detail_type: String,
    #[serde(rename = "EventBusName")]
    event_bus_name: Option<String>,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Resources")]
    resources: Option<Vec<String>>,
    #[serde(rename = "Time")]
    time: Option<DateTime<Utc>>,
    #[serde(rename = "TraceHeader")]
    trace_header: Option<String>,
}

/// Provides conversion from a JSON string to a PutEventsRequestEntry instance.
impl FromStr for PutEventsRequestEntry {
    type Err = serde_json::Error;

    /// Creates a PutEventsRequestEntry from a JSON string.
    ///
    /// # Parameters
    /// * `string` - A JSON string representation of a PutEventsRequestEntry
    ///
    /// # Returns
    /// A new PutEventsRequestEntry instance
    ///
    /// # Panics
    /// Panics if the string cannot be parsed as valid JSON or doesn't match the expected structure.
    /// In production code, consider using a method that returns a Result instead of unwrapping.
    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(_s)
    }
}

#[cfg(test)]
mod put_events_request_entry_test {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_from_str() {
        let entry_str = "{\n         \"Source\":\"com.mycompany.myapp\",\n         \"Detail\":\"{ \\\"key1\\\": \\\"value1\\\", \\\"key2\\\": \\\"value2\\\" }\",\n         \"Resources\":[\n            \"resource1\",\n            \"resource2\"\n         ],\n         \"DetailType\":\"myDetailType\"\n      }";
        let entry = PutEventsRequestEntry {
            detail: "{ \"key1\": \"value1\", \"key2\": \"value2\" }".to_string(),
            detail_type: "myDetailType".to_string(),
            event_bus_name: None,
            source: "com.mycompany.myapp".to_string(),
            resources: Some(vec!["resource1".to_string(), "resource2".to_string()]),
            time: None,
            trace_header: None,
        };

        let entry_from_str = PutEventsRequestEntry::from_str(entry_str);

        assert_eq!(entry_from_str.unwrap(), entry);
    }
}
