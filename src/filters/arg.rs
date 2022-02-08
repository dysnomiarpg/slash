use std::{pin::Pin, task::Poll};

use futures_util::Future;

use crate::{
    filter::{FilterBase, Internal},
    generic::One,
    reject::Rejection,
};

#[derive(Debug, Clone)]
pub struct StringArgument {
    name: String,
    description: String,
}

impl FilterBase for StringArgument {
    type Extract = One<String>;
    type Error = Rejection;
    type Future = StringArgFut;

    fn filter(&self, _: Internal) -> Self::Future {
        todo!()
    }
}

pub struct StringArgFut {}

impl Future for StringArgFut {
    type Output = Result<One<String>, Rejection>;

    fn poll(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        Poll::Ready(Ok(("".to_string(),)))
    }
}

/// Create a new string argument.
pub fn string<S: AsRef<str>>(name: S, description: S) -> StringArgument {
    StringArgument {
        name: name.as_ref().to_string(),
        description: description.as_ref().to_string(),
    }
}

pub struct IntArg {
    name: String,
    description: String,
}

/// Create a new integer argument.
pub fn int(name: String, description: String) -> IntArg {
    IntArg { name, description }
}

pub struct FloatArg {
    name: String,
    description: String,
}

/// Create a new float argument.
pub fn float(name: String, description: String) -> FloatArg {
    FloatArg { name, description }
}
