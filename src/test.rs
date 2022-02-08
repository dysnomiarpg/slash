use std::{error::Error, future::Future};

trait Executor: FnOnce() -> dyn Future<Output = Result<(), Box<dyn Error>>> {}

struct Test {
    inner: Box<dyn Executor>,
}

impl Test {
    fn new<E: Executor + 'static>(inner: E) -> Self {
        Self {
            inner: Box::new(inner),
        }
    }
    async fn execute(&self) {}
}

fn test() {
    let s = Test::new(|| async { Ok(()) });
}
