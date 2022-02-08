use std::{pin::Pin, task::Poll};

use futures_util::Future;

use crate::{
    context::Context,
    filter::{FilterBase, Internal},
    generic::One,
    reject::Rejection,
};

/// Returns a new command object.
pub fn command(name: &str) -> Command {
    Command {
        name: name.to_string(),
        description: "".to_string(),
    }
}

pub struct Command {
    name: String,
    description: String,
}

impl Command {
    /// Set the name of this command.
    pub fn name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = name.as_ref().to_string();
        self
    }
    /// Set the description of this command.
    pub fn description<S: AsRef<str>>(mut self, description: S) -> Self {
        self.description = description.as_ref().to_string();
        self
    }
}

impl FilterBase for Command {
    type Extract = One<Context>;
    type Error = Rejection;
    type Future = CommandFut;

    fn filter(&self, _: Internal) -> Self::Future {
        todo!()
    }
}

pub struct CommandFut {}

impl Future for CommandFut {
    type Output = Result<One<Context>, Rejection>;

    fn poll(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Poll::Ready(Ok((Context::new(),)))
    }
}
