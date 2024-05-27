use super::{
    error::RustSideError, http_client::http_client::HttpClient, network_antenna::NetworkAntenna,
};
use crate::{
    log::{Context, Filter, Log, Message},
    logger::LoggingStrategy,
};
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
};
use uniffi::Object;

#[derive(Object)]
pub struct TransferClient {
    pub http_client: Option<HttpClient>,
}

#[uniffi::export]
impl TransferClient {
    #[uniffi::constructor]
    pub fn new(
        strategy: LoggingStrategy,
        network_antenna: Option<Arc<dyn NetworkAntenna>>,
    ) -> Arc<Self> {
        match strategy {
            LoggingStrategy::Local => Arc::new(Self { http_client: None }),
            LoggingStrategy::Cloud(_) => Arc::new(Self {
                http_client: Some(HttpClient {
                    network_antenna: network_antenna
                        .expect("Network Antenna should be present if strategy is Cloud"),
                }),
            }),
        }
    }
}

impl TransferClient {
    pub async fn write_log(
        &self,
        log: Log,
        strategy: LoggingStrategy,
    ) -> Result<String, RustSideError> {
        match strategy {
            LoggingStrategy::Local => {
                let mut buffer = File::create("log.txt").map_err(|err| RustSideError::IOError {
                    error: err.to_string(),
                })?;
                let serialized_log = bincode::serialize(&log).expect("Serialize log");
                buffer
                    .write(&serialized_log)
                    .map_err(|err| RustSideError::IOError {
                        error: err.to_string(),
                    })
                    .map(|x| x.to_string())
            }
            LoggingStrategy::Cloud(provider) => Ok({
                provider
                    .transmit_log(log)
                    .await
                    .map_err(|err| RustSideError::NetworkingError {
                        error: err.to_string(),
                    })?
            }),
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
        network_antenna::transfer_client::{Placeholder, TransferClient},
    };

    // #[test]
    // fn write_to_file() {
    //     let placeholder = Log::placeholder();
    //     let client = TransferClient::new(LoggingStrategy::Local, None);
    //     client.write_log(placeholder).await.expect("Write log to file");
    //     assert!(File::open("log.txt").is_ok())
    // }

    // #[test]
    // fn read_message_from_file() {
    //     write_to_file();
    //     let client = TransferClient::new(LoggingStrategy::Local, None);
    //     let log = client
    //         .retrieve_logs(crate::log::Filter::Text)
    //         .expect("Retrieve and deserialize log");
    //     assert_eq!(log.message.message, "Placeholder Log".to_owned())
    // }
}
