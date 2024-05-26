use super::{error::CommonError, network_antenna::NetworkAntenna, network_request::NetworkRequest};
use crate::{
    log::{Context, Filter, Log, Message},
    logger::LoggingStrategy,
};
use serde::Deserialize;
use std::{
    any::type_name,
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
    ) -> Arc<Self> {
        match strategy {
            LoggingStrategy::Local => Arc::new(Self {
                strategy,
                http_client: None,
            }),
            LoggingStrategy::Cloud(_) => Arc::new(Self {
                strategy,
                http_client: Some(HttpClient {
                    network_antenna: network_antenna
                        .expect("Network Antenna should be present if strategy is Cloud"),
                }),
            }),
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
    pub fn write_log(&self, log: Log) -> Result<usize, std::io::Error> {
        match self.strategy {
            LoggingStrategy::Local => {
                let mut buffer = File::create("log.txt")?;
                let serialized_log = bincode::serialize(&log).expect("Serialize log");
                buffer.write(&serialized_log)
            }
            LoggingStrategy::Cloud(_) => todo!(),
        }
    }

    pub fn retrieve_logs(&self, filter: Filter) -> Result<Log, std::io::Error> {
        let mut f = File::open("log.txt")?;
        let mut buffer: Vec<u8> = vec![];

        let _ = f.read_to_end(&mut buffer)?;
        let log = bincode::deserialize::<Log>(&buffer).expect("Deserialize Log");
        println!("The deserialize log: {:?}", &log);

        Ok(log)
    }
}

pub trait Placeholder {
    fn placeholder() -> Self;
}

impl Placeholder for Log {
    fn placeholder() -> Self {
        Self::new(
            Context { ctx: String::new() },
            Message::new("Placeholder Log".to_string()),
            crate::log::Level::Normal,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use crate::{
        log::Log,
        logger::LoggingStrategy,
        network_antenna::client::{Client, Placeholder},
    };

    #[test]
    fn write_to_file() {
        let placeholder = Log::placeholder();
        let client = Client::new(LoggingStrategy::Local, None);
        client.write_log(placeholder).expect("Write log to file");
        assert!(File::open("log.txt").is_ok())
    }

    #[test]
    fn read_message_from_file() {
        write_to_file();
        let client = Client::new(LoggingStrategy::Local, None);
        let log = client
            .retrieve_logs(crate::log::Filter::Text)
            .expect("Retrieve and deserialize log");
        assert_eq!(log.message.message, "Placeholder Log".to_owned())
    }
}
