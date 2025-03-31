use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::{EdgeeRequest, Event};

pub trait ApiRequest {
    fn get_edgee_request(&self, settings_map: &Settings, event: &Event) -> EdgeeRequest;
}
