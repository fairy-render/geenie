use std::future::Future;

use crate::{Context, GeenieError};

pub trait Func<'a, E, C, I> {
    fn call(
        self,
        context: Context<'a, E, C>,
        input: I,
    ) -> impl Future<Output = Result<(), GeenieError>>;
}

impl<'a, I, E, C, T> Func<'a, E, C, I> for T
where
    C: 'a,
    E: 'a,
    T: FnOnce(Context<'a, E, C>, I) -> Result<(), GeenieError>,
{
    fn call(
        self,
        context: Context<'a, E, C>,
        input: I,
    ) -> impl Future<Output = Result<(), GeenieError>> {
        async move { (self)(context, input) }
    }
}
