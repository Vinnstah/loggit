use std::{fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Log {
    pub context: Context,
    pub message: Message,
    pub level: Level,
    pub timestamp: SystemTime,
}

impl Log {
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
        write!(
            f,
            "[{}] {:#?}: {}",
            self.level, self.timestamp, self.message
        )
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Context {
    pub ctx: String,
}

impl Context {
    pub fn new(ctx: String) -> Self {
        Self { ctx }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Level {
    Debug,
    Trace,
    Normal,
    Error,
    Critical,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
}
