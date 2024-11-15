use std::collections::BTreeSet;
use std::path::Path;

use relative_path::RelativePathBuf;

use crate::command::DynamicCommand;
use crate::{command::CommandList, FileList};
use crate::{File, GeenieError, Item};

#[derive(Default)]
pub(crate) struct ResultBuilder {
    files: Vec<File>,
    seen: BTreeSet<RelativePathBuf>,
    commands: Vec<Box<dyn DynamicCommand>>,
}

impl ResultBuilder {
    pub fn push_file(&mut self, file: File) -> Result<(), GeenieError> {
        if self.seen.contains(&file.path) {
            return Err(GeenieError::duplicate(file.path.clone()));
        }

        self.seen.insert(file.path.clone());
        self.files.push(file);

        Ok(())
    }

    pub fn push_command(&mut self, command: Box<dyn DynamicCommand>) {
        self.commands.push(command);
    }

    pub fn build(self) -> GeenieResult {
        GeenieResult {
            files: FileList { files: self.files },
            commands: self.commands.into(),
        }
    }
}

pub struct GeenieResult {
    pub files: FileList,
    pub commands: CommandList,
}

impl GeenieResult {
    #[cfg(feature = "fs")]
    pub async fn write_to(&self, path: impl AsRef<Path>, force: bool) -> Result<(), GeenieError> {
        self.files.write_to(path.as_ref(), force).await?;
        self.commands.run_in(path.as_ref()).await?;

        Ok(())
    }
}

impl<C> Item<C> for GeenieResult {
    fn process<'a>(
        self,
        mut ctx: crate::Context<'a, C>,
    ) -> impl std::future::Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            ctx.push(self.files).push(self.commands);

            Ok(())
        }
    }
}
