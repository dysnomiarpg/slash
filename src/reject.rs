use std::convert::Infallible;

pub trait IsReject: Send {}

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

#[derive(Debug)]
pub struct Rejection {}

impl IsReject for Rejection {}

impl IsReject for Infallible {}
