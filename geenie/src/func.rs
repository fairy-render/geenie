use std::future::Future;

use crate::{Context, GeenieError};

pub trait Func<'a, C, I> {
    fn call(
        self,
        context: Context<'a, C>,
        input: I,
    ) -> impl Future<Output = Result<(), GeenieError>>;
}

impl<'a, I, C, T> Func<'a, C, I> for T
where
    C: 'a,
    T: FnOnce(Context<'a, C>, I) -> Result<(), GeenieError>,
{
    fn call(
        self,
        context: Context<'a, C>,
        input: I,
    ) -> impl Future<Output = Result<(), GeenieError>> {
        async move { (self)(context, input) }
    }
}
