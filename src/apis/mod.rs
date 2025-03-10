use crate::apis::put_events_api::PutEventsApi;
use crate::apis::utils::api_provider::ApiProvider;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event};

pub mod put_events_api;
mod utils;


#[derive(Debug, Copy, Clone, PartialEq)]
enum ApiVariant {
    PutEvents(PutEventsApi)
}

pub struct ApiRequestBuilder <T: ApiProvider> {
    api: Option<T>,
    settings_map: Option<Settings>,
    event: Option<Event>,
}

impl<T: ApiProvider> ApiRequestBuilder<T> {
    pub fn new() -> Self {
        ApiRequestBuilder {
            api: None,
            settings_map: None,
            event: None
        }
    }

    pub fn api(mut self, api: T) -> ApiRequestBuilder<T>
    where
        T: ApiProvider
    {
        self.api = Some(api);
        self
    }

    pub fn settings(mut self, settings_dict: &Dict) -> ApiRequestBuilder<T> {
        let settings = Settings::new(settings_dict).unwrap();
        let api_variant = settings.api_variant.clone();
        let api = Self::get_api(&api_variant);

        self.settings_map = Some(settings);
        match &self.api {
            Some(_T) => {},
            None => {
                self.api = Some(api);
            }
        }

        self
    }

    pub fn build(self) -> EdgeeRequest {
        let event = self.event.unwrap();
        let settings = self.settings_map.unwrap();
        self.api.unwrap().get_edgee_request(&settings, &event)
    }


    pub fn event(mut self, event: &Event) -> ApiRequestBuilder<T> {
        self.event = Some(event.clone());
        self
    }

    fn get_api(api_variant: &ApiVariant) -> T
    where
        T: ApiProvider
    {
       match api_variant {
           ApiVariant::PutEvents(PutEventsApi) => PutEventsApi
       }
    }


}
