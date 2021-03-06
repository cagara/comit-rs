use crate::{api::IntoFrame, json};
use serde::de::DeserializeOwned;
use serde_json::{self, Value as JsonValue};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize)]
pub struct Request {
    #[serde(rename = "type")]
    _type: String,
    headers: HashMap<String, JsonValue>,
    body: JsonValue,
}

impl Request {
    pub fn new(_type: String, headers: HashMap<String, JsonValue>, body: JsonValue) -> Self {
        Request {
            _type,
            headers,
            body,
        }
    }

    pub fn get_header<H: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Option<Result<H, serde_json::Error>> {
        self.headers
            .get(key)
            .map(|header| H::deserialize(header.clone()))
    }

    pub fn get_body<B: DeserializeOwned>(&self) -> Option<Result<B, serde_json::Error>> {
        if self.body.is_null() {
            return None;
        }

        Some(B::deserialize(self.body.clone()))
    }
}

impl IntoFrame<json::Frame> for Request {
    fn into_frame(self, id: u32) -> json::Frame {
        // Serializing Request should never fail because its members are just Strings
        // and JsonValues
        let payload = serde_json::to_value(self).unwrap();

        json::Frame::new("REQUEST".into(), id, payload)
    }
}
