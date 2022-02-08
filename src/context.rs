//! Defines the `Context` struct, the primary method through which bots are
//! capable of interacting with the Discord API.

use std::cell::RefCell;

use scoped_tls::scoped_thread_local;

scoped_thread_local!(
    /// Thread-scoped context value.
    static CONTEXT: RefCell<Context>
);

/// The primary context of a command.
pub struct Context {}

pub(crate) fn set<F, U>(r: &RefCell<Context>, func: F) -> U
where
    F: FnOnce() -> U,
{
    CONTEXT.set(r, func)
}

pub(crate) fn is_set() -> bool {
    CONTEXT.is_set()
}

pub(crate) fn with<F, R>(func: F) -> R
where
    F: FnOnce(&mut Context) -> R,
{
    CONTEXT.with(move |route| func(&mut *route.borrow_mut()))
}
