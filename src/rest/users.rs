use const_format::formatcp;

use crate::{model::snowflake::Snowflake, rest::API_ENDPOINT};

const USER_API_ENDPOINT: &str = formatcp!("{}/users", API_ENDPOINT);

/// Create a URL to look up the current account's profile.
pub fn me() -> String {
    format!("{}/@me", USER_API_ENDPOINT)
}

/// Create a URL to look up a user's profile.
pub fn id<S: Into<Snowflake>>(id: S) -> String {
    format!("{}/{}", USER_API_ENDPOINT, id.into())
}
