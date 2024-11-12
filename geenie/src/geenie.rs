use core::{future::Future, pin::Pin};

use crate::{
    file::{FileList, FileListBuilder},
    item::{DynamicItem, ItemBox},
    question::DynamicQuestion,
    Context, GeenieError, Item,
};

#[derive(Default)]
pub struct Geenie {
    items: Vec<Box<dyn DynamicItem>>,
}

impl Geenie {
    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item + 'static,
    {
        self.items.push(Box::new(ItemBox(item)));
        self
    }

    pub async fn run(&self) -> Result<FileList, GeenieError> {
        let mut files = FileListBuilder::default();
        for item in &self.items {
            self.process_item(item, &mut files).await?;
        }

        Ok(files.build())
    }

    async fn process_item(
        &self,
        item: &Box<dyn DynamicItem>,
        files: &mut FileListBuilder,
    ) -> Result<(), GeenieError> {
        let mut questions = Vec::default();

        item.process(Context {
            files,
            questions: &mut questions,
        })
        .await?;

        for question in questions {
            self.process_question(question, files).await?;
        }

        Ok(())
    }

    fn process_question<'a>(
        &'a self,
        item: Box<dyn DynamicQuestion>,
        files: &'a mut FileListBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move {
            let mut questions = Vec::default();

            item.process(Context {
                files,
                questions: &mut questions,
            })
            .await?;

            for question in questions {
                self.process_question(question, files).await?;
            }

            Ok(())
        })
    }
}

impl Item for Geenie {
    fn process<'a>(
        &'a self,
        ctx: Context<'a>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            for item in &self.items {
                item.process(Context {
                    files: ctx.files,
                    questions: ctx.questions,
                })
                .await?;
            }

            Ok(())
        }
    }
}
