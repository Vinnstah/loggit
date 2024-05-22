#![feature(trait_upcasting)]

pub mod log;
pub mod logger;
pub mod analytics;
pub mod network_antenna;

uniffi::include_scaffolding!("loggit");