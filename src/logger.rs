use async_trait::async_trait;
use std::fmt::Debug;
use std::sync::Arc;
use uniffi::{Enum, Object};

use crate::{
    log::{Context, Level, Log, Message},
    network_antenna::{
        error::{CommonError, RustSideError},
        transfer_client::TransferClient,
    },
};

#[derive(Object)]
pub struct Logger {
    pub transfer_client: TransferClient,
    pub storage_strategy: LoggingStrategy,
}

#[uniffi::export]
impl Logger {
    #[uniffi::method]
    pub async fn log(
        &self,
        context: Context,
        message: Message,
        level: Level,
    ) -> Result<String, RustSideError> {
        let log = Log::new(context, message, level);
        self.transfer_client
            .write_log(log, self.storage_strategy.clone())
            .await
            .map_err(|err| RustSideError::IOError {
                error: err.to_string(),
            })
            .map(|x| format!("Bytes written: {}", x))
    }
}

#[derive(Debug, Clone, Enum)]
pub enum LoggingStrategy {
    Local,
    Cloud(Arc<dyn Provider>),
}

#[uniffi::export(with_foreign)]
#[async_trait]
pub trait Provider: Send + Sync + Debug {
    async fn transmit_log(&self, log: Log) -> Result<String, CommonError>;
}

#[cfg(test)]
mod tests {}
