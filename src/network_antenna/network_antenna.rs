use std::collections::HashMap;

use uniffi::Record;

pub trait NetworkAntenna: Send + Sync {
    async fn make_request(
        &self,
        request: FFINetworkingRequest,
    ) -> Result<FFINetworkingResponse, FFINetworkingError>;
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingRequest {
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,

    pub body: Vec<u8>,
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct FFINetworkingResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: Vec<u8>,
}

// impl From<FFINetworkingError> for FFIBridgeError {
//     fn from(value: FFINetworkingError) -> Self {
//         Self::FromFFI {
//             error: value.into(),
//         }
//     }
// }

pub enum FFIBridgeError {}
pub enum FFINetworkingError {}

