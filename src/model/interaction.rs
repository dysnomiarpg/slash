use std::collections::HashMap;

use serde::Deserialize;

use super::{command::ApplicationCommandType, member::Member, snowflake::Snowflake, user::User};

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub(crate) enum InteractionType {
    #[serde(rename = "PING")]
    Ping = 1,
    #[serde(rename = "APPLICATION_COMMAND")]
    ApplicationCommand,
    #[serde(rename = "MESSAGE_COMPONENT")]
    MessageComponent,
    #[serde(rename = "APPLICATION_COMMAND_AUTOCOMPLETE")]
    ApplicationCommandAutocomplete,
}

#[derive(Debug, Clone, Deserialize)]
struct ResolvedData {
    pub users: HashMap<Snowflake, User>,
    pub members: HashMap<Snowflake, Member>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub(crate) enum InteractionData {
    ApplicationCommand {
        id: Snowflake,
        name: String,
        ty: ApplicationCommandType,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub ty: InteractionType,
}
