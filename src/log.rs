use std::{fmt::{Debug, Display}, time::SystemTime};

use serde::{Deserialize, Serialize};
use uniffi::{Enum, Record};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Record)]
pub struct Log {
    pub context: Context,
    pub message: Message,
    pub level: Level,
    pub timestamp: SystemTime,
}

#[uniffi::export]
impl Log {
    #[uniffi::constructor]
    pub fn new(context: Context, message: Message, level: Level) -> Self {
        Self {
            context,
            message,
            level,
            timestamp: SystemTime::now(),
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {:#?}: {}", self.level, self.timestamp, self.message)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Record)]
pub struct Context {
    pub ctx: String,
}

impl Context {
    pub fn new(ctx: String) -> Self {
        Self { ctx }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Enum)]
pub enum Level {
    Debug,
    Trace,
    Normal,
    Error,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Enum)]
pub enum Filter {
    Time,
    Context,
    Text,
    Level(Level)
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Debug => write!(f, "Debug"),
            Level::Trace => write!(f, "Trace"),
            Level::Normal => write!(f, "Normal"),
            Level::Error => write!(f, "Error"),
            Level::Critical => write!(f, "Critical"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Record)]
pub struct Message {
    id: String,
    pub message: String,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Message {
    pub fn new(message: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_log_empty_message() {
        let log = Log::new(
            Context::new("Onboarding".to_owned()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.message.message, "".to_owned());
    }

    #[test]
    fn new_log_ctx() {
        let log = Log::new(
            Context::new("Onboarding".to_owned()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.context.ctx, "Onboarding".to_owned());
    }

    #[test]
    fn new_log_level() {
        let log = Log::new(
            Context::new("Onboarding".to_owned()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.level, Level::Normal);
    }

    #[test]
    fn display_level() {
        let log = Log::new(
            Context::new(String::new()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.level.to_string(), "Normal".to_owned());
    }

    #[test]
    fn display_context() {
        let log = Log::new(
            Context::new("Context".to_owned()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.context.ctx.to_string(), "Context".to_owned());
    }

    #[test]
    fn display_log() {
        let log = Log::new(
            Context::new("Context".to_owned()),
            Message::new(String::new()),
            Level::Normal,
        );
        assert_eq!(log.to_string(), format!("[Normal] {:#?}: ", log.timestamp));
    }
}
