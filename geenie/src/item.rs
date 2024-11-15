use relative_path::RelativePathBuf;

use crate::{result::ResultBuilder, Context, File, GeenieError};
use core::{future::Future, pin::Pin};

pub trait Item<E, C> {
    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

impl<T, E, C> Item<E, C> for T
where
    T: 'static,
    for<'a> T: FnOnce(Context<'a, E, C>) -> Result<(), GeenieError>,
{
    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move { (self)(ctx) }
    }
}

pub trait ItemExt<E, C>: Item<E, C> {
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

impl<T, E, C> ItemExt<E, C> for T where T: Item<E, C> {}

pub trait DynamicItem<E, C> {
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a, E, C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct ItemBox<T>(pub T);

impl<T, E, C> DynamicItem<E, C> for ItemBox<T>
where
    T: Item<E, C> + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a, E, C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move { self.0.process(ctx).await })
    }
}

impl<E, C> Item<E, C> for ItemBox<Box<dyn DynamicItem<E, C>>> {
    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
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

impl<T, E, C> Item<E, C> for MountItem<T>
where
    C: 'static,
    E: Clone + 'static,
    T: Item<E, C> + 'static,
{
    fn process<'a>(
        self,
        mut ctx: Context<'a, E, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let mut files = ResultBuilder::default();
            let mut items = Vec::default();

            self.item
                .process(Context {
                    files: &mut files,
                    questions: &mut items,
                    ctx: ctx.ctx,
                    env: ctx.env,
                })
                .await?;

            let ret = files.build(ctx.env.clone());

            for file in ret.files {
                ctx.file(File {
                    path: self.mount.join(file.path),
                    content: file.content,
                })?;
            }

            for cmd in ret.commands {
                ctx.files.push_command(cmd);
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
