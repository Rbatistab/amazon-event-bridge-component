//! Event Bridge API request handling module
//!
//! This module provides structures and implementations for building and handling
//! Amazon EventBridge API requests.

use crate::apis::put_events_api::PutEventsApi;
use crate::apis::utils::api_request::ApiRequest;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event};

pub mod put_events_api;
mod utils;

/// Type alias for a boxed trait object implementing the ApiRequest trait
type EdgeeRequester = Box<dyn ApiRequest>;

/// Represents the different types of API operations supported
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ApiVariant {
    /// PutEvents API operation variant
    PutEvents(PutEventsApi),
}

/// Builder pattern implementation for creating API requests
pub struct ApiRequestBuilder {
    /// The type of API operation to be performed
    api_variant: Option<ApiVariant>,
    /// Configuration settings for the API request
    settings_map: Option<Settings>,
    /// Event data to be included in the request
    event: Option<Event>,
}

impl ApiRequestBuilder {
    /// Creates a new instance of ApiRequestBuilder with default values
    ///
    /// # Returns
    ///
    /// * A new `ApiRequestBuilder` instance with all fields set to `None`
    pub fn new() -> Self {
        ApiRequestBuilder {
            api_variant: None,
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
    pub fn settings(mut self, settings_dict: &Dict) -> ApiRequestBuilder {
        let settings = Settings::new(settings_dict).unwrap();
        let api_variant = settings.api_variant;

        self.settings_map = Some(settings);
        self.api_variant = Some(api_variant);

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
    pub fn event(mut self, event: &Event) -> ApiRequestBuilder {
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
        let api: EdgeeRequester = self.get_api_from_api_variant();
        let event = self.event.unwrap();
        let settings = self.settings_map.unwrap();

        api.get_edgee_request(&settings, &event)
    }

    /// Creates the appropriate API requester based on the configured API variant
    ///
    /// # Returns
    ///
    /// * `EdgeeRequester` - A boxed trait object implementing the ApiRequest trait
    ///
    /// # Panics
    ///
    /// Will panic if the api_variant is not set (None)
    fn get_api_from_api_variant(&self) -> EdgeeRequester {
        match self.api_variant.unwrap() {
            ApiVariant::PutEvents(PutEventsApi) => Box::new(PutEventsApi),
        }
    }
}
