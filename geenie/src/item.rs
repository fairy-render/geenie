use relative_path::RelativePathBuf;

use crate::{file::FileListBuilder, Context, File, GeenieError};
use core::{future::Future, pin::Pin};

pub trait Item<C> {
    fn process<'a>(self, ctx: Context<'a, C>)
        -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

impl<T, C> Item<C> for T
where
    T: 'static,
    for<'a> T: FnOnce(Context<'a, C>) -> Result<(), GeenieError>,
{
    fn process<'a>(
        self,
        ctx: Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move { (self)(ctx) }
    }
}

pub trait ItemExt<C>: Item<C> {
    fn mount<P>(self, path: P) -> MountItem<Self>
    where
        Self: Sized,
        P: Into<RelativePathBuf>,
    {
        MountItem {
            item: self,
            mount: path.into(),
        }
    }
}

impl<T, C> ItemExt<C> for T where T: Item<C> {}

pub trait DynamicItem<C> {
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a, C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct ItemBox<T>(pub T);

impl<T, C> DynamicItem<C> for ItemBox<T>
where
    T: Item<C> + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a, C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move { self.0.process(ctx).await })
    }
}

impl<C> Item<C> for ItemBox<Box<dyn DynamicItem<C>>> {
    fn process<'a>(
        self,
        ctx: Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move { self.0.process(ctx).await }
    }
}

pub struct MountItem<T> {
    item: T,
    mount: RelativePathBuf,
}

impl<T> MountItem<T> {
    pub fn new(mount: impl Into<RelativePathBuf>, item: T) -> MountItem<T> {
        MountItem {
            item,
            mount: mount.into(),
        }
    }
}

impl<T, C> Item<C> for MountItem<T>
where
    C: 'static,
    T: Item<C> + 'static,
{
    fn process<'a>(
        self,
        mut ctx: Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let mut files = FileListBuilder::default();
            let mut items = Vec::default();

            self.item
                .process(Context {
                    files: &mut files,
                    questions: &mut items,
                    ctx: ctx.ctx,
                })
                .await?;

            for file in files.build() {
                ctx.file(File {
                    path: self.mount.join(file.path),
                    content: file.content,
                })?;
            }

            for item in items {
                ctx.push(MountItem {
                    item: ItemBox(item),
                    mount: self.mount.clone(),
                });
            }

            Ok(())
        }
    }
}
