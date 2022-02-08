use const_format::formatcp;

use crate::{model::snowflake::Snowflake, rest::API_ENDPOINT};

const APPLICATION_API_ENDPOINT: &str = formatcp!("{}/applications", API_ENDPOINT);

/// Create a URL to look up an application by its ID.
pub fn id<S: Into<Snowflake>>(application_id: S) -> String {
    format!("{}/{}", APPLICATION_API_ENDPOINT, application_id.into())
}

/// Create a URL to look up an application's commands.
pub fn commands<S: Into<Snowflake>>(application_id: S) -> String {
    format!("{}/commands", id(application_id))
}

/// Create a URL to look up all guilds in this application.
pub fn guilds<S: Into<Snowflake>>(application_id: S) -> String {
    format!("{}/guilds", id(application_id))
}

/// Create a URL to look up a guild in the application.
pub fn guild<S: Into<Snowflake>>(application_id: S, guild_id: S) -> String {
    format!("{}/guilds/{}", guilds(application_id), guild_id.into())
}

/// Create a URL to look up a guild's commands.
pub fn guild_commands<S: Into<Snowflake>>(application_id: S, guild_id: S) -> String {
    format!(
        "{}/guilds/{}/commands",
        guilds(application_id),
        guild_id.into()
    )
}
