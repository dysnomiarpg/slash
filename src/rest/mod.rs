//! Defines REST API endpoints for the Discord REST API.
pub mod applications;
pub mod users;

/// The root-level API endpoint.
pub(crate) const API_ENDPOINT: &'static str = "https://discordapp.com/api/v9";
