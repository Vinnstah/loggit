use super::error::CommonError;
use crate::UniffiCustomTypeConverter;
use serde::Serialize;
use std::collections::HashMap;
use url::Url;

uniffi::custom_type!(Url, String);

impl UniffiCustomTypeConverter for Url {
    type Builtin = String;

    fn into_custom(val: Self::Builtin) -> uniffi::Result<Self> {
        Ok(Url::parse(&val)?)
    }

    fn from_custom(obj: Self) -> Self::Builtin {
        obj.into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, uniffi::Record)]
pub struct NetworkRequest {
    pub url: Url,
    pub method: NetworkMethod,
    pub headers: HashMap<String, String>,

    pub body: Vec<u8>,
}

impl NetworkRequest {
    pub fn new_post(url: Url) -> Self {
        Self {
            url,
            method: NetworkMethod::Post,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body.into();
        self
    }

    pub fn with_serializing_body<T: Serialize>(self, body: T) -> uniffi::Result<Self> {
        let serialized =
            serde_json::to_vec(&body).map_err(|_| CommonError::FailedToSerializeToJSON)?;

        Ok(self.with_body(serialized))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, uniffi::Enum)]
pub enum NetworkMethod {
    Post,
    Get,
}
