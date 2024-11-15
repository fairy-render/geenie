use std::{future::Future, path::Path, pin::Pin};

use crate::{GeenieError, Item};

pub trait Command {
    fn run<'a>(&'a self, path: &'a Path) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

pub trait DynamicCommand {
    fn run<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct CommandBox<T>(pub T);

impl<T> DynamicCommand for CommandBox<T>
where
    T: Command + 'static,
{
    fn run<'a>(
        &'a self,
        path: &'a Path,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move { self.0.run(path).await })
    }
}

pub struct CommandList {
    cmds: Vec<Box<dyn DynamicCommand>>,
}

impl CommandList {
    pub async fn run_in(&self, path: &Path) -> Result<(), GeenieError> {
        for cmd in &self.cmds {
            cmd.run(path).await?;
        }
        Ok(())
    }
}

impl From<Vec<Box<dyn DynamicCommand>>> for CommandList {
    fn from(value: Vec<Box<dyn DynamicCommand>>) -> Self {
        CommandList { cmds: value }
    }
}

impl IntoIterator for CommandList {
    type IntoIter = std::vec::IntoIter<Box<dyn DynamicCommand>>;
    type Item = Box<dyn DynamicCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.cmds.into_iter()
    }
}

impl<'a> IntoIterator for &'a CommandList {
    type IntoIter = std::slice::Iter<'a, Box<dyn DynamicCommand>>;
    type Item = &'a Box<dyn DynamicCommand>;

    fn into_iter(self) -> Self::IntoIter {
        self.cmds.iter()
    }
}

impl<C> Item<C> for CommandList {
    fn process<'a>(
        self,
        ctx: crate::Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            for cmd in self {
                ctx.files.push_command(cmd);
            }

            Ok(())
        }
    }
}

pub struct CommandItem<T>(pub T);

impl<C, T> Item<C> for CommandItem<T>
where
    T: Command + 'static,
{
    fn process<'a>(
        self,
        mut ctx: crate::Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            ctx.command(self.0);
            Ok(())
        }
    }
}
