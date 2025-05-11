use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event};
use exports::edgee::components::data_collection::Guest;
use crate::put_events_api::PutEventsRequestBuilder;

pub mod put_events_api;

wit_bindgen::generate!({
    world: "data-collection",
    path: "wit",
    additional_derives: [serde::Serialize],
    generate_all
});
export!(Component);

struct Component;

impl Guest for Component {
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        send_to_event_bridge(edgee_event, settings_dict)
    }

    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        send_to_event_bridge(edgee_event, settings_dict)
    }

    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        send_to_event_bridge(edgee_event, settings_dict)
    }
}

fn send_to_event_bridge(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
    let edgee_api_request = PutEventsRequestBuilder::new()
        .settings(&settings_dict)
        .event(&edgee_event)
        .build();

    Ok(edgee_api_request)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     use crate::exports::edgee::components::data_collection::{
//         Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
//     };
//     use exports::edgee::components::data_collection::Consent;
//     use pretty_assertions::assert_eq;
//     use uuid::Uuid;
//
//     fn get_context() -> Context {
//         Context {
//
//         }
//     }
//
//     fn get_event() -> Event {
//         Event {
//             uuid: Uuid::new_v4().to_string(),
//             timestamp: 123,
//             timestamp_millis: 123,
//             timestamp_micros: 123,
//             event_type: EventType::Page,
//             data: Data::Page(sample_page_data()),
//             context: sample_context(edgee_id, locale, session_start),
//             consent,
//         }
//         // {
//         //     "EndpointId": "string",
//         //     "Entries": [
//         //     {
//         //         "Detail": "string",
//         //         "DetailType": "string",
//         //         "EventBusName": "string",
//         //         "Resources": [ "string" ],
//         //         "Source": "string",
//         //         "Time": number,
//         //         "TraceHeader": "string"
//         //     }
//         //     ]
//         // }
//     }
//
//     fn get_settings_dict() -> Dict {
//         vec![
//             ("aws_region".to_string(), "us-west-2".to_string()),
//             ("aws_access_key".to_string(), "Rufus".to_string()),
//             ("aws_secret_key".to_string(), "Rufus".to_string()),
//             ("domain".to_string(), "amazon.com".to_string()),
//             ("aws_security_token".to_string(), "Rufus".to_string()),
//         ]
//     }
//
//     #[test]
//     fn page_works_fine() {
//         // TBD
//         assert_eq!(true, true);
//     }
// }
