use std::future::Future;

use crate::{Context, GeenieError};

pub trait Func<'a, I> {
    fn call(self, context: Context<'a>, input: I) -> impl Future<Output = Result<(), GeenieError>>;
}

impl<'a, I, T> Func<'a, I> for T
where
    T: FnOnce(Context<'a>, I) -> Result<(), GeenieError>,
{
    fn call(self, context: Context<'a>, input: I) -> impl Future<Output = Result<(), GeenieError>> {
        async move { (self)(context, input) }
    }
}
