use serde::Deserialize;

use super::{snowflake::Snowflake, user::User};

#[derive(Debug, Clone, Deserialize)]
pub struct Member {
    /// The user this guild member represents.
    pub user: User,
    #[serde(rename = "nick")]
    /// This user's guild nickname.
    pub nickname: String,
    /// The member's guild avatar hash.
    pub avatar: Option<String>,
    /// Array of role object ids.
    pub roles: Vec<Snowflake>,
    /// When the user joined the guild.
    pub joined_at: String,
    /// When the user started boosting the guild
    pub premium_since: String,
    /// Whether the user is deafened in voice channels.
    pub deaf: bool,
    /// Whether the user is muted in voice channels.
    pub mute: bool,
    /// Whether the user has not yet passed the guild's Membership Screening requirements.
    pub pending: Option<bool>,
    /// Total permissions of the member in the channel, including overwrites, returned when in the interaction object
    pub permissions: String,
    /// When the user's timeout will expire and the user will be able to communicate in the guild again,
    /// null or a time in the past if the user is not timed out
    pub communication_disabled_until: Option<String>,
}
