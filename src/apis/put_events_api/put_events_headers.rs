use aws_credential_types::Credentials;
use aws_sigv4::http_request::{
    sign, SignableBody, SignableRequest, SigningParams, SigningSettings,
};
use aws_smithy_runtime_api::client::identity::Identity;
use std::time::SystemTime;
use aws_sigv4::sign::v4;
use crate::apis::utils::settings::Settings;
use crate::exports::edgee::components::data_collection::Dict;

pub struct PutEventsHeaders {
    region: String,
    access_key: String,
    secret_key: String,
    security_token: Option<String>
}

impl PutEventsHeaders {

    pub fn new(settings_map: &Settings) -> PutEventsHeaders {
      Self {
          region: settings_map.region.clone(),
          access_key: settings_map.access_key.clone(),
          secret_key: settings_map.secret_key.clone(),
          security_token: settings_map.security_token.clone()
      }
    }

    pub fn get_headers_host(&self) -> String {
        format!(
            "https://events.{}.amazon.com",
            self.region.clone()
        )
    }

    /// Gets headers for PutEvents API call
    ///
    // POST / HTTP/1.1
    // Host: events.<region>.<domain>
    // x-amz-Date: <Date>
    // Authorization: AWS4-HMAC-SHA256 Credential=<Credential>, SignedHeaders=content-type;date;host;user-agent;x-amz-date;x-amz-target;x-amzn-requestid, Signature=<Signature>
    // User-Agent: <UserAgentString>
    // Content-Type: application/x-amz-json-1.1
    // Content-Length: <PayloadSizeBytes>
    // Connection: Keep-Alive
    pub fn get_headers(
        &self,
        body: &String
    ) -> Dict {
        let identity: Identity = Credentials::new(
            self.access_key.clone(),
            self.secret_key.clone(),
            self.security_token.clone(),
            None,
            "hardcoded-credentials"
        ).into();

        let signing_settings = SigningSettings::default();

        let signing_params: SigningParams = v4::SigningParams::builder()
            .identity(&identity)
            .region(self.region.as_str())
            .name("event-bridge")
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .unwrap()
            .into();

        let signable_request = SignableRequest::new(
            "POST",
            self.get_headers_host(),
            std::iter::empty(),
            SignableBody::Bytes(body.as_bytes())
        ).expect("Signable request");

        // generate the signature headers
        let (signing_instructions, _signature) = sign(signable_request, &signing_params)
            .unwrap()
            .into_parts();

        // convert to Vec<(String, String)>
        signing_instructions
            .headers()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()

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
