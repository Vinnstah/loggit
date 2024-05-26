use uniffi::{Enum, Object};

use crate::{
    log::{Context, Level, Log, Message},
    network_antenna::{transfer_client::TransferClient, error::RustSideError},
};

#[derive(Object)]
pub struct Logger {
    pub transfer_client: TransferClient,
}

#[uniffi::export]
impl Logger {
    #[uniffi::method]
    pub fn log(
        &self,
        context: Context,
        message: Message,
        level: Level,
    ) -> Result<String, RustSideError> {
        let log = Log::new(context, message, level);
        self.transfer_client
            .write_log(log)
            .map_err(|err| RustSideError::IOError {
                error: err.to_string(),
            })
            .map(|x| format!("Bytes written: {}", x))
    }
}

#[derive(Debug, Clone, Enum, PartialEq, Eq)]
pub enum LoggingStrategy {
    Local,
    Cloud(Provider),
}

#[derive(Debug, Clone, Enum, PartialEq, Eq)]
pub enum Provider {
    Azure,
    AWS,
    GCP,
}

#[cfg(test)]
mod tests {}
