use crate::log::Log;
use std::{
    fs::File,
    io::{Read, Write},
    sync::Arc,
    vec,
};

pub trait Logger {
    fn log(&self, gateway: Arc<impl Gateway>, log: Log) -> ();
}

pub trait Gateway {
    fn upload_log(&self, log: Log) -> Result<usize, std::io::Error>;
    fn retrieve_logs(&self) -> Result<Vec<Log>, std::io::Error>;
    fn logging_strategy(&self) -> &LoggingStrategy;
}

#[derive(Debug, Clone)]
pub enum LoggingStrategy {
    Local,
    Cloud(Provider),
}

#[derive(Debug, Clone)]
pub enum Provider {
    Azure,
    AWS,
    GCP,
}

#[derive(Clone)]
pub struct Client {
    pub strategy: LoggingStrategy,
}

impl Client {
    pub fn new(strategy: LoggingStrategy) -> Self {
        Self { strategy }
    }
}

impl Gateway for Client {
    fn upload_log(&self, log: Log) -> Result<usize, std::io::Error> {
        let mut buffer = File::create("log.txt")?;
        buffer.write(log.to_string().as_bytes())
    }

    fn retrieve_logs(&self) -> Result<Vec<Log>, std::io::Error> {
        let mut f = File::open("log.txt")?;
        let mut buffer = [0; 10];

        // read up to 10 bytes
        let n = f.read(&mut buffer[..])?;

        println!("The bytes: {:?}", &buffer[..n]);

        Ok(vec![])
    }

    fn logging_strategy(&self) -> &LoggingStrategy {
        &self.strategy
    }
}

impl Logger for Client {
    fn log(&self, gateway: Arc<impl Gateway>, log: Log) -> () {
        gateway.upload_log(log).expect("");
    }
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
