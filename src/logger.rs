use uniffi::{Enum, Record};

#[derive(Debug, Clone, Record, PartialEq, Eq)]
pub struct Logger {}

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
