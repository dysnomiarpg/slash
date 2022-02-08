use std::{convert::Infallible, pin::Pin, task::Poll};

use futures_util::{future, Future};

use crate::{
    filter::{FilterBase, Internal},
    Filter,
};


/// Returns a new command object.
pub fn command(name: &str) -> impl Filter<Extract = (), Error = Infallible> {
    Command { name: name.to_string() }
}

pub(crate) struct Command {
    name: String,
}

impl Command {
	pub fn name<S: AsRef<str>>(mut self, name: S) -> Self {
		self.name = name.as_ref().to_string();
		self
	}
}

impl FilterBase for Command {
    type Extract = ();
    type Error = Infallible;
    type Future = CommandFut;

    fn filter(&self, _: Internal) -> Self::Future {
        todo!()
    }
}

pub(crate) struct CommandFut {}

impl Future for CommandFut {
    type Output = Result<(), Infallible>;

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}
