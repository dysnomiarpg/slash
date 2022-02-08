use serde::{Deserialize, Serialize};

use super::snowflake::Snowflake;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChoiceValue {
    String(String),
    Integer(i64),
    Double(f64),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CommandOptionChoice {
    name: String,
    value: ChoiceValue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CommandOptionType {
    #[serde(rename = "SUB_COMMAND")]
    SubCommand = 1,
    #[serde(rename = "SUB_COMMAND_GROUP")]
    SubCommandGroup,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "USER")]
    User,
    #[serde(rename = "CHANNEL")]
    Channel,
    #[serde(rename = "ROLE")]
    Role,
    #[serde(rename = "MENTIONABLE")]
    Mentionable,
    #[serde(rename = "NUMBER")]
    Number,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApplicationCommandOption {
    /// The type of option
    #[serde(rename = "type")]
    pub ty: CommandOptionType,
    /// The name of the option
    pub name: String,
    /// The description of the option, 1-100 characters.
    pub description: String,
    /// If the parameter is required or optional--default false
    #[serde(default)]
    pub required: bool,
    /// Choices for `STRING`, `INTEGER`, and `NUMBER` types for the user to pick from, max 25.
    pub choices: Vec<CommandOptionChoice>,
    /// If the option is a subcommand or subcommand group type, these nested options will be the parameters.
    pub options: Vec<ApplicationCommandOption>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ApplicationCommandType {
    /// Slash commands; a text-based command that shows up when a user types `/`.
    #[serde(rename = "CHAT_INPUT")]
    ChatInput = 1,
    /// A UI-based command that shows up when you right click or tap on a user.
    #[serde(rename = "USER")]
    User,
    /// A UI-based command that shows up when you right click or tap on a message
    #[serde(rename = "MESSAGE")]
    Message,
}

impl Default for ApplicationCommandType {
    fn default() -> Self {
        ApplicationCommandType::ChatInput
    }
}

/// Application commands are commands that an application can register to Discord.
/// They provide users a first-class way of interacting directly with your application
/// that feels deeply integrated into Discord.
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommand {
    /// Unique id of the command.
    pub id: Snowflake,
    /// The type of command, defaults 1 if not set.
    #[serde(rename = "type", default)]
    pub ty: ApplicationCommandType,
    /// Unique id of the parent application
    pub application_id: Snowflake,
    /// Guild id of the command, if not global
    pub guild_id: Option<Snowflake>,
    /// 1-32 character name
    pub name: String,
    /// 1-100 character description for `CHAT_INPUT` commands, empty string for `USER` and `MESSAGE` commands.
    pub description: String,
    /// The parameters for the command, max 25, valid only for `CHAT_INPUT` commands.
    pub options: Vec<ApplicationCommandOption>,
}
