use std::{error::Error, future::Future, pin::Pin};

use serde::{Deserialize, Serialize};

use crate::model::{command::ApplicationCommandType, snowflake::Snowflake};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ChoiceValue {
    String(String),
    Integer(i64),
    Double(f64),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct OptionChoice {
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
pub struct CommandOption {
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
    pub choices: Vec<OptionChoice>,
    /// If the option is a subcommand or subcommand group type, these nested options will be the parameters.
    pub options: Vec<CommandOption>,
}

/// Shared metadata between the three command types.
#[derive(Clone, Debug)]
pub struct CommandMeta {
    /// The name of this command.
    pub name: String,
    /// 1-100 character description for `CHAT_INPUT` commands, empty string for `USER` and `MESSAGE` commands.
    pub description: String,
    /// The id of the guild this command is for.
    pub guild_id: Option<Snowflake>,
    /// Whether the command is enabled by default when the app is added to a guild.
    pub default_permission: bool,
}

/// Trait representing an `ApplicationCommand`.
pub trait Command: Sized {
    /// Get the command meta data.
    fn meta(&self) -> CommandMeta;
    /// Get the command type.
    fn ty(&self) -> ApplicationCommandType;
}

pub struct CommandFuture;

impl Future for CommandFuture {
    type Output = Result<(), Box<dyn Error>>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}

trait CommandExecutor: Fn() -> CommandFuture {}
type PinnedExecutor = Pin<Box<dyn CommandExecutor>>;

pub struct ChatInputCommand<F>
where
    F: Future,
{
    meta: CommandMeta,
    executor: fn() -> F,
    /// The parameters for the command, max 25
    pub options: Vec<CommandOption>,
}

impl<F: Future> Command for ChatInputCommand<F> {
    fn meta(&self) -> CommandMeta {
        self.meta.clone()
    }
    fn ty(&self) -> ApplicationCommandType {
        ApplicationCommandType::ChatInput
    }
}

pub struct MessageCommand {
    meta: CommandMeta,
    executor: Box<dyn Fn() -> Result<(), Box<dyn Error>>>,
}

impl Command for MessageCommand {
    fn meta(&self) -> CommandMeta {
        self.meta.clone()
    }
    fn ty(&self) -> ApplicationCommandType {
        ApplicationCommandType::ChatInput
    }
}

pub struct UserCommand {
    meta: CommandMeta,
    executor: Box<dyn Fn() -> Result<(), Box<dyn Error>>>,
}

impl Command for UserCommand {
    fn meta(&self) -> CommandMeta {
        self.meta.clone()
    }
    fn ty(&self) -> ApplicationCommandType {
        ApplicationCommandType::ChatInput
    }
}

pub trait CommandBuilder<T: Command, F: Future> {
    /// Create a new builder.
    fn new() -> Self;
    /// Consume this builder and return the built command.
    fn build(self) -> Result<T, Box<dyn Error>>;
    /// Set the name of the command.
    fn set_name<S: AsRef<str>>(self, name: S) -> Self;
    /// Set the description of the command.
    fn set_description<S: AsRef<str>>(self, description: S) -> Self;
    /// Set the guild id of the command.
    fn set_guild_id<S: Into<Snowflake>>(self, guild_id: S) -> Self;
    /// Set the default permission of the command.
    fn set_default_permission(self, default_permission: bool) -> Self;
    fn on_execute(self, on_execute: fn() -> F) -> Self;
}

pub struct ChatInputCommandBuilder<F: Future> {
    inner: ChatInputCommand<F>,
}

impl<F: Future> CommandBuilder<ChatInputCommand<F>, F> for ChatInputCommandBuilder<F> {
    fn new() -> Self {
        Self {
            inner: ChatInputCommand {
                meta: CommandMeta {
                    guild_id: None,
                    name: String::new(),
                    description: String::new(),
                    default_permission: false,
                },
                options: vec![],
                executor: || async { Ok(()) },
            },
        }
    }
    fn build(self) -> Result<ChatInputCommand<F>, Box<dyn Error>> {
        Ok(self.inner)
    }
    fn set_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.inner.meta.name = name.as_ref().to_string();
        self
    }
    fn set_description<S: AsRef<str>>(mut self, description: S) -> Self {
        self.inner.meta.description = description.as_ref().to_string();
        self
    }
    fn set_guild_id<S: Into<Snowflake>>(mut self, guild_id: S) -> Self {
        self.inner.meta.guild_id = Some(guild_id.into());
        self
    }
    fn set_default_permission(mut self, default_permission: bool) -> Self {
        self.inner.meta.default_permission = default_permission;
        self
    }
    fn on_execute(mut self, on_execute: fn() -> F) -> Self {
        self.inner.executor = on_execute;
        self
    }
}

impl<F: Future> ChatInputCommandBuilder<F> {}

pub struct MessageCommandBuilder {
    inner: MessageCommand,
}

impl CommandBuilder<MessageCommand> for MessageCommandBuilder {
    fn new() -> Self {
        Self {
            inner: MessageCommand {
                meta: CommandMeta {
                    guild_id: None,
                    name: String::new(),
                    description: String::new(),
                    default_permission: false,
                },
                executor: Box::new(|| Ok(())),
            },
        }
    }
    fn build(self) -> Result<MessageCommand, Box<dyn Error>> {
        Ok(self.inner)
    }
    fn set_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.inner.meta.name = name.as_ref().to_string();
        self
    }
    fn set_description<S: AsRef<str>>(mut self, description: S) -> Self {
        self.inner.meta.description = description.as_ref().to_string();
        self
    }
    fn set_guild_id<S: Into<Snowflake>>(mut self, guild_id: S) -> Self {
        self.inner.meta.guild_id = Some(guild_id.into());
        self
    }
    fn set_default_permission(mut self, default_permission: bool) -> Self {
        self.inner.meta.default_permission = default_permission;
        self
    }
    fn on_execute<F: Fn() -> Result<(), Box<dyn Error>> + 'static>(
        mut self,
        on_execute: F,
    ) -> Self {
        self.inner.executor = Box::new(on_execute);
        self
    }
}
