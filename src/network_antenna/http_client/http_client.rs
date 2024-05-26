use std::{any::type_name, sync::Arc};
use serde::Deserialize;
use crate::network_antenna::{error::CommonError, network_antenna::NetworkAntenna};
use super::network_request::NetworkRequest;

pub struct HttpClient {
    pub network_antenna: Arc<dyn NetworkAntenna>,
}

impl HttpClient {
    pub fn new(network_antenna: Arc<dyn NetworkAntenna>) -> Self {
        Self { network_antenna }
    }
}

impl HttpClient {
    pub async fn execute_network_request(
        &self,
        request: NetworkRequest,
    ) -> Result<Vec<u8>, CommonError> {
        let response = self
            .network_antenna
            .execute_network_request(request)
            .await
            .map_err(|err| CommonError::FromNetworkingError { from: err })?;

        // Check for valid status code
        if !(200..=299).contains(&response.status_code) {
            return Err(CommonError::NetworkResponseBadCode);
        }

        Ok(response.body)
    }
}

impl HttpClient {
    fn model_from_response<U>(&self, bytes: Vec<u8>) -> Result<U, CommonError>
    where
        U: for<'a> Deserialize<'a>,
    {
        serde_json::from_slice::<U>(&bytes).map_err(|_| {
            CommonError::NetworkResponseJSONDeserialize {
                into_type: type_name::<U>().to_string(),
            }
        })
    }

    pub async fn execute_request_with_decoding<U>(
        &self,
        request: NetworkRequest,
    ) -> Result<U, CommonError>
    where
        U: for<'a> Deserialize<'a>,
    {
        let response = self.execute_network_request(request).await?;
        self.model_from_response(response)
    }
}
