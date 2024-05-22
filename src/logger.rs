use uniffi::{Enum, Object, Record};

use crate::{log::Log, network_antenna::network_antenna::NetworkAntenna};
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
    vec,
};

// pub trait Logger {
//     fn log(&self, gateway: Arc<impl Gateway>, log: Log) -> ();
// }

// pub trait Gateway {
//     fn upload_log(&self, log: Log) -> Result<usize, std::io::Error>;
//     fn retrieve_logs(&self) -> Result<Vec<Log>, std::io::Error>;
//     fn logging_strategy(&self) -> &LoggingStrategy;
// }

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
mod tests {

    use std::borrow::Borrow;

    use crate::log::{self, Message};

    use super::*;

    // #[test]
    // fn log_to_file() {
    //     let client = Client::new();
    //     client.log(
    //         client.clone().into(),
    //         Log::new(
    //             log::Context { ctx: String::new() },
    //             Message::new("Nej".to_string()),
    //             log::Level::Error
    //         )
    //     );
    //     let mut f = File::open("log.txt").unwrap();
    //     let mut buffer = Vec::new();
    //     assert_eq!(

    //         // read the whole file
    //         f.read_to_end(&mut buffer).unwrap()
    //         ,
    //         "[Error] SystemTime {
    //             tv_sec: 1716323270,
    //             tv_nsec: 155837000,
    //         }: Nej".to_owned()
    //     );
    // }
}
