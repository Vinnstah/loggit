use super::{error::CommonError, network_antenna::NetworkAntenna, network_request::NetworkRequest};
use crate::{
    log::{Filter, Log},
    logger::LoggingStrategy,
};
use serde::Deserialize;
use std::{
    any::type_name,
    error::Error,
    fs::File,
    io::{Read, Write},
    sync::Arc,
};
use uniffi::Object;

#[derive(Object)]
pub struct Client {
    pub strategy: LoggingStrategy,
    pub http_client: Option<HttpClient>,
}

#[uniffi::export]
impl Client {
    #[uniffi::constructor]
    pub fn new(
        strategy: LoggingStrategy,
        network_antenna: Option<Arc<dyn NetworkAntenna>>,
    ) -> Self {
        match strategy {
            LoggingStrategy::Local => Self {
                strategy,
                http_client: None,
            },
            LoggingStrategy::Cloud(_) => Self {
                strategy,
                http_client: Some(HttpClient {
                    network_antenna: network_antenna
                        .expect("Network Antenna should be present if strategy is Cloud"),
                }),
            },
        }
    }
}

pub struct HttpClient {
    network_antenna: Arc<dyn NetworkAntenna>,
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

impl Client {
    fn write_log(&self, log: Log) -> Result<usize, std::io::Error> {
        match self.strategy {
            LoggingStrategy::Local => {
                let mut buffer = File::create("log.txt")?;
                buffer.write(log.to_string().as_bytes())
            }
            LoggingStrategy::Cloud(_) => todo!(),
        }
    }

    fn retrieve_logs(&self, filter: Filter) -> Result<Vec<Log>, std::io::Error> {
        let mut f = File::open("log.txt")?;
        let mut buffer = [0; 10];

        // read up to 10 bytes
        let n = f.read(&mut buffer[..])?;

        println!("The bytes: {:?}", &buffer[..n]);

        Ok(vec![])
    }
}

// impl Client {
//     fn log(&self, gateway: Arc<impl Gateway>, log: Log) -> () {
//         gateway.upload_log(log).expect("");
//     }
// }