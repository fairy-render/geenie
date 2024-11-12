use crate::{Context, GeenieError};
use core::{future::Future, pin::Pin};

pub trait Item {
    fn process<'a>(self, ctx: Context<'a>) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

impl<T> Item for T
where
    T: 'static,
    for<'a> T: FnOnce(Context<'a>) -> Result<(), GeenieError>,
{
    fn process<'a>(self, ctx: Context<'a>) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move { (self)(ctx) }
    }
}

pub trait DynamicItem {
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct ItemBox<T>(pub T);

impl<T> DynamicItem for ItemBox<T>
where
    T: Item + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move { self.0.process(ctx).await })
    }
}
