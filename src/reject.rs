//! Rejections
//!
//! Part of the power of the [`Filter`](../trait.Filter.html) system is being able to
//! reject a request from a filter chain. This allows for filters to be
//! combined with `or`, so that if one side of the chain finds that a request
//! doesn't fulfill its requirements, the other side can try to process
//! the request.
//!
//! Many of the built-in [`filters`](../filters) will automatically reject
//! the request with an appropriate rejection. However, you can also build
//! new custom [`Filter`](../trait.Filter.html)s and still want other routes to be
//! matchable in the case a predicate doesn't hold.
//!
//! As a request is processed by a Filter chain, the rejections are accumulated into
//! a list contained by the [`Rejection`](struct.Rejection.html) type. Rejections from
//! filters can be handled using [`Filter::recover`](../trait.Filter.html#method.recover).
//! This is a convenient way to map rejections into a [`Reply`](../reply/trait.Reply.html).
//!
//! For a more complete example see the
//! [Rejection Example](https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs)
//! from the repository.
//!
//! # Example
//!
//! ```
//! use warp::{reply, Reply, Filter, reject, Rejection, http::StatusCode};
//!
//! #[derive(Debug)]
//! struct InvalidParameter;
//!
//! impl reject::Reject for InvalidParameter {}
//!
//! // Custom rejection handler that maps rejections into responses.
//! async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
//!     if err.is_not_found() {
//!         Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
//!     } else if let Some(e) = err.find::<InvalidParameter>() {
//!         Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
//!     } else {
//!         eprintln!("unhandled rejection: {:?}", err);
//!         Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
//!     }
//! }
//!
//!
//! // Filter on `/:id`, but reject with InvalidParameter if the `id` is `0`.
//! // Recover from this rejection using a custom rejection handler.
//! let route = warp::path::param()
//!     .and_then(|id: u32| async move {
//!         if id == 0 {
//!             Err(warp::reject::custom(InvalidParameter))
//!         } else {
//!             Ok("id is valid")
//!         }
//!     })
//!     .recover(handle_rejection);
//! ```

use std::any::Any;
use std::convert::Infallible;
use std::fmt;

pub(crate) use self::sealed::{CombineRejection, IsReject};


/// Rejects a request with a custom cause.
///
/// A [`recover`][] filter should convert this `Rejection` into a `Reply`,
/// or else this will be returned as a `500 Internal Server Error`.
///
/// [`recover`]: ../trait.Filter.html#method.recover
pub fn custom<T: Reject>(err: T) -> Rejection {
    Rejection::custom(Box::new(err))
}

/// Protect against re-rejecting a rejection.
///
/// ```compile_fail
/// fn with(r: warp::Rejection) {
///     let _wat = warp::reject::custom(r);
/// }
/// ```
fn __reject_custom_compilefail() {}

/// A marker trait to ensure proper types are used for custom rejections.
///
/// Can be converted into Rejection.
///
/// # Example
///
/// ```
/// use warp::{Filter, reject::Reject};
///
/// #[derive(Debug)]
/// struct RateLimited;
///
/// impl Reject for RateLimited {}
///
/// let route = warp::any().and_then(|| async {
///     Err::<(), _>(warp::reject::custom(RateLimited))
/// });
/// ```
// Require `Sized` for now to prevent passing a `Box<dyn Reject>`, since we
// would be double-boxing it, and the downcasting wouldn't work as expected.
pub trait Reject: fmt::Debug + Sized + Send + Sync + 'static {}

trait Cause: fmt::Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Cause for T
where
    T: fmt::Debug + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn Cause {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub(crate) fn known<T: Into<Known>>(err: T) -> Rejection {
    Rejection::known(err.into())
}

/// Rejection of a request by a [`Filter`](crate::Filter).
///
/// See the [`reject`](module@crate::reject) documentation for more.
pub struct Rejection {
    reason: Reason,
}

enum Reason {
    NotFound,
    Other(Box<Rejections>),
}

enum Rejections {
    Known(Known),
    Custom(Box<dyn Cause>),
    Combined(Box<Rejections>, Box<Rejections>),
}

#[derive(Debug)]
pub(crate) enum Known {
	DiscordApiError(DiscordApiError),
}

impl Known {
	fn inner_as_any(&self) -> &dyn Any {
		match *self {
			Known::DiscordApiError(ref e) => e
		}
	}
}

#[derive(Debug)]
pub(crate) struct DiscordApiError {}

impl Rejection {
    fn known(known: Known) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Known(known))),
        }
    }

    fn custom(other: Box<dyn Cause>) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Custom(other))),
        }
    }

    /// Searches this `Rejection` for a specific cause.
    ///
    /// A `Rejection` will accumulate causes over a `Filter` chain. This method
    /// can search through them and return the first cause of this type.
    ///
    /// # Example
    ///
    /// ```
    /// #[derive(Debug)]
    /// struct Nope;
    ///
    /// impl warp::reject::Reject for Nope {}
    ///
    /// let reject = warp::reject::custom(Nope);
    ///
    /// if let Some(nope) = reject.find::<Nope>() {
    ///    println!("found it: {:?}", nope);
    /// }
    /// ```
    pub fn find<T: 'static>(&self) -> Option<&T> {
        if let Reason::Other(ref rejections) = self.reason {
            return rejections.find();
        }
        None
    }

    /// Returns true if this Rejection was made via `warp::reject::not_found`.
    ///
    /// # Example
    ///
    /// ```
    /// let rejection = warp::reject();
    ///
    /// assert!(rejection.is_not_found());
    /// ```
    pub fn is_not_found(&self) -> bool {
        matches!(self.reason, Reason::NotFound)
    }
}

impl<T: Reject> From<T> for Rejection {
    #[inline]
    fn from(err: T) -> Rejection {
        custom(err)
    }
}

impl From<Infallible> for Rejection {
    #[inline]
    fn from(infallible: Infallible) -> Rejection {
        match infallible {}
    }
}

impl IsReject for Infallible {}

impl IsReject for Rejection {}

impl fmt::Debug for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Rejection").field(&self.reason).finish()
    }
}

impl fmt::Debug for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Reason::NotFound => f.write_str("NotFound"),
            Reason::Other(ref other) => match **other {
                Rejections::Known(ref e) => fmt::Debug::fmt(e, f),
                Rejections::Custom(ref e) => fmt::Debug::fmt(e, f),
                Rejections::Combined(ref a, ref b) => {
                    let mut list = f.debug_list();
                    a.debug_list(&mut list);
                    b.debug_list(&mut list);
                    list.finish()
                }
            },
        }
    }
}

// ===== Rejections =====

impl Rejections {
    fn find<T: 'static>(&self) -> Option<&T> {
        match *self {
            Rejections::Known(ref e) => e.inner_as_any().downcast_ref(),
            Rejections::Custom(ref e) => e.downcast_ref(),
            Rejections::Combined(ref a, ref b) => a.find().or_else(|| b.find()),
        }
    }

    fn debug_list(&self, f: &mut fmt::DebugList<'_, '_>) {
        match *self {
            Rejections::Known(ref e) => {
                f.entry(e);
            }
            Rejections::Custom(ref e) => {
                f.entry(e);
            }
            Rejections::Combined(ref a, ref b) => {
                a.debug_list(f);
                b.debug_list(f);
            }
        }
    }
}

mod sealed {
    use super::{Reason, Rejection, Rejections};
    use std::convert::Infallible;
    use std::fmt;

    // This sealed trait exists to allow Filters to return either `Rejection`
    // or `!`. There are no other types that make sense, and so it is sealed.
    pub trait IsReject: fmt::Debug + Send + Sync {}

    fn _assert_object_safe() {
        fn _assert(_: &dyn IsReject) {}
    }

    // This weird trait is to allow optimizations of propagating when a
    // rejection can *never* happen (currently with the `Never` type,
    // eventually to be replaced with `!`).
    //
    // Using this trait means the `Never` gets propagated to chained filters,
    // allowing LLVM to eliminate more code paths. Without it, such as just
    // requiring that `Rejection::from(Never)` were used in those filters,
    // would mean that links later in the chain may assume a rejection *could*
    // happen, and no longer eliminate those branches.
    pub trait CombineRejection<E>: Send + Sized {
        /// The type that should be returned when only 1 of the two
        /// "rejections" occurs.
        ///
        /// # For example:
        ///
        /// `warp::any().and(warp::path("foo"))` has the following steps:
        ///
        /// 1. Since this is `and`, only **one** of the rejections will occur,
        ///    and as soon as it does, it will be returned.
        /// 2. `warp::any()` rejects with `Never`. So, it will never return `Never`.
        /// 3. `warp::path()` rejects with `Rejection`. It may return `Rejection`.
        ///
        /// Thus, if the above filter rejects, it will definitely be `Rejection`.
        type One: IsReject + From<Self> + From<E> + Into<Rejection>;

        /// The type that should be returned when both rejections occur,
        /// and need to be combined.
        type Combined: IsReject;

        fn combine(self, other: E) -> Self::Combined;
    }

    impl CombineRejection<Rejection> for Rejection {
        type One = Rejection;
        type Combined = Rejection;

        fn combine(self, other: Rejection) -> Self::Combined {
            let reason = match (self.reason, other.reason) {
                (Reason::Other(left), Reason::Other(right)) => {
                    Reason::Other(Box::new(Rejections::Combined(left, right)))
                }
                (Reason::Other(other), Reason::NotFound)
                | (Reason::NotFound, Reason::Other(other)) => {
                    // ignore the NotFound
                    Reason::Other(other)
                }
                (Reason::NotFound, Reason::NotFound) => Reason::NotFound,
            };

            Rejection { reason }
        }
    }

    impl CombineRejection<Infallible> for Rejection {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, other: Infallible) -> Self::Combined {
            match other {}
        }
    }

    impl CombineRejection<Rejection> for Infallible {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, _: Rejection) -> Self::Combined {
            match self {}
        }
    }

    impl CombineRejection<Infallible> for Infallible {
        type One = Infallible;
        type Combined = Infallible;

        fn combine(self, _: Infallible) -> Self::Combined {
            match self {}
        }
    }
}
