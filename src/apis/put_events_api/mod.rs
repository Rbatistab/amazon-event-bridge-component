use crate::apis::put_events_api::put_events_body::PutEventsBody;
use crate::apis::put_events_api::put_events_headers::PutEventsHeaders;
use crate::apis::utils::api_provider::ApiProvider;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::{EdgeeRequest, Event, HttpMethod};

pub mod put_events_headers;
pub mod put_events_body;
mod put_events_body_entry;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PutEventsApi;

impl ApiProvider for PutEventsApi {
    fn get_edgee_request(
        &self,
        settings_map: &Settings,
        event: &Event
    ) -> EdgeeRequest {

        let put_events_headers = PutEventsHeaders::new(settings_map);

        let method = HttpMethod::Post;
        let url = put_events_headers.get_headers_host();
        let body = PutEventsBody::get_body(event);
        let headers = put_events_headers.get_headers(&body);

        EdgeeRequest {
            method,
            url,
            headers,
            body,
            forward_client_headers: false // Hardcoding this one for now
        }
    }
}
