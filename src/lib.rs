pub mod error;
pub mod cli;
pub mod core;
pub mod utils;
pub mod gui;

pub use crate::error::AirLinkError;
pub type Result<T> = std::result::Result<T, AirLinkError>;