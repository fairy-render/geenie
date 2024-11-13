use relative_path::RelativePathBuf;

use crate::{file::FileListBuilder, Context, File, GeenieError};
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

pub trait ItemExt: Item {
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

impl<T> ItemExt for T where T: Item {}

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

impl Item for ItemBox<Box<dyn DynamicItem>> {
    fn process<'a>(self, ctx: Context<'a>) -> impl Future<Output = Result<(), GeenieError>> + 'a {
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

impl<T> Item for MountItem<T>
where
    T: Item + 'static,
{
    fn process<'a>(
        self,
        mut ctx: Context<'a>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let mut files = FileListBuilder::default();
            let mut items = Vec::default();

            self.item
                .process(Context {
                    files: &mut files,
                    questions: &mut items,
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
