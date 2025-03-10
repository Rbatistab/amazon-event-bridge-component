use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event};
use exports::edgee::components::data_collection::Guest;
use crate::apis::ApiRequestBuilder;

mod apis;

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

    let edgee_api_request = ApiRequestBuilder::new()
        .settings(&settings_dict)
        .event(&edgee_event)
        .build();

    Ok(edgee_api_request)
}


#[cfg(test)]
mod tests {
    use super::*;

    // use crate::exports::edgee::components::data_collection::{
    //     Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    // };
    // use exports::edgee::components::data_collection::Consent;
    // use pretty_assertions::assert_eq;
    // use uuid::Uuid;

    #[test]
    fn page_works_fine() {
        // TBD
        assert_eq!(true, true);
    }
}
