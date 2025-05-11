//! Amazon EventBridge PutEvents API implementation module
//!
//! This module provides the implementation for making PutEvents API requests
//! to Amazon EventBridge. It handles request construction including headers,
//! body formatting, and proper AWS authentication.
//! 
//! Event Bridge API request handling module
//!
//! This module provides structures and implementations for building and handling
//! Amazon EventBridge API requests.

mod settings;
mod put_events_headers;

use crate::put_events_api::settings::Settings;
use crate::put_events_api::put_events_headers::PutEventsHeaders;
use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};


/// Represents the PutEvents API request handler
///
/// This struct implements the `ApiRequest` trait to provide
/// functionality for making PutEvents requests to Amazon EventBridge.
#[derive(Debug)]
pub struct PutEventsApi;

impl PutEventsApi {
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
    fn get_edgee_request(settings_map: &Settings, event: &Event) -> EdgeeRequest {
        let put_events_headers = PutEventsHeaders::new(settings_map);

        let method = HttpMethod::Post;
        let url = put_events_headers.get_headers_host();
        let body = serde_json::to_string(&event).unwrap_or_default();
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

/// Builder pattern implementation for PutEventsAPI
pub struct PutEventsRequestBuilder {
    /// Configuration settings for the API request
    settings_map: Option<Settings>,
    /// Event data to be included in the request
    event: Option<Event>,
}
impl PutEventsRequestBuilder {
    /// Creates a new instance of ApiRequestBuilder with default values
    ///
    /// # Returns
    ///
    /// * A new `ApiRequestBuilder` instance with all fields set to `None`
    pub fn new() -> Self {
        PutEventsRequestBuilder {
            settings_map: None,
            event: None,
        }
    }

    /// Configures the settings for the API request
    ///
    /// # Arguments
    ///
    /// * `settings_dict` - A dictionary containing configuration settings
    ///
    /// # Returns
    ///
    /// * `ApiRequestBuilder` - Returns self for method chaining
    ///
    /// # Panics
    ///
    /// Will panic if the settings dictionary cannot be converted to Settings
    pub fn settings(mut self, settings_dict: &Dict) -> PutEventsRequestBuilder {
        let settings = Settings::new(settings_dict).unwrap();
        self.settings_map = Some(settings);
        self
    }

    /// Sets the event data for the API request
    ///
    /// # Arguments
    ///
    /// * `event` - The event data to be included in the request
    ///
    /// # Returns
    ///
    /// * `ApiRequestBuilder` - Returns self for method chaining
    pub fn event(mut self, event: &Event) -> PutEventsRequestBuilder {
        self.event = Some(event.clone());
        self
    }

    /// Builds the final EdgeeRequest from the configured builder
    ///
    /// # Returns
    ///
    /// * `EdgeeRequest` - The constructed request ready for sending
    ///
    /// # Panics
    ///
    /// Will panic if:
    /// * The event is not set (None)
    /// * The settings are not configured (None)
    /// * The api_variant is not set (None)
    pub fn build(self) -> EdgeeRequest {
        let event = self.event.unwrap();
        let settings = self.settings_map.unwrap();

        PutEventsApi::get_edgee_request(&settings, &event)
    }
}
