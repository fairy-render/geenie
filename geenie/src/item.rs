use crate::{Context, GeenieError};
use core::{future::Future, pin::Pin};

pub trait Item {
    fn process<'a>(
        &'a self,
        ctx: Context<'a>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

impl<T> Item for T
where
    for<'a> T: Fn(Context<'a>) -> Result<(), GeenieError>,
{
    fn process<'a>(
        &'a self,
        ctx: Context<'a>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move { (self)(ctx) }
    }
}

pub trait DynamicItem {
    fn process<'a>(
        &'a self,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct ItemBox<T>(pub T);

impl<T> DynamicItem for ItemBox<T>
where
    T: Item,
{
    fn process<'a>(
        &'a self,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move { self.0.process(ctx).await })
    }
}
