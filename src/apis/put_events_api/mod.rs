//! Amazon EventBridge PutEvents API implementation module
//!
//! This module provides the implementation for making PutEvents API requests
//! to Amazon EventBridge. It handles request construction including headers,
//! body formatting, and proper AWS authentication.

use crate::apis::put_events_api::put_events_body::PutEventsBody;
use crate::apis::put_events_api::put_events_headers::PutEventsHeaders;
use crate::apis::utils::api_request::ApiRequest;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::{EdgeeRequest, Event, HttpMethod};

pub mod put_events_body;
mod put_events_body_entry;
pub mod put_events_headers;

/// Represents the PutEvents API request handler
///
/// This struct implements the `ApiRequest` trait to provide
/// functionality for making PutEvents requests to Amazon EventBridge.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PutEventsApi;

impl ApiRequest for PutEventsApi {
    /// Creates an EdgeeRequest configured for the PutEvents API
    ///
    /// # Arguments
    ///
    /// * `settings_map` - Configuration settings including AWS credentials and endpoint information
    /// * `event` - The event data to be sent to EventBridge
    ///
    /// # Returns
    ///
    /// * `EdgeeRequest` - A fully configured request ready to be sent to EventBridge
    fn get_edgee_request(&self, settings_map: &Settings, event: &Event) -> EdgeeRequest {
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
            forward_client_headers: false, // Hardcoding this one for now
        }
    }
}
