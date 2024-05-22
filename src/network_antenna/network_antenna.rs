use crate::network_antenna::error::FFIBridgeError;
use uniffi::Record;

use super::{error::NetworkingError, network_request::NetworkRequest};

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait NetworkAntenna: Send + Sync {
    async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<NetworkingResponse, NetworkingError>;
}

#[derive(Record, Clone, Debug, PartialEq, Eq)]
pub struct NetworkingResponse {
    pub status_code: u16,

    /// Can be empty.
    pub body: Vec<u8>,
}

impl From<NetworkingError> for FFIBridgeError {
    fn from(value: NetworkingError) -> Self {
        Self::FromFFI {
            error: value.into(),
        }
    }
}
