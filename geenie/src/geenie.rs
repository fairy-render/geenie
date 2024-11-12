use core::{future::Future, pin::Pin};

use crate::{
    file::{FileList, FileListBuilder},
    item::{DynamicItem, ItemBox},
    question::QuestionBox,
    Context, GeenieError, Item, Question,
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

    pub fn ask<T>(&mut self, question: T) -> &mut Self
    where
        T: Question + 'static,
    {
        self.items.push(Box::new(QuestionBox(question)));
        self
    }

    pub async fn run(self) -> Result<FileList, GeenieError> {
        let mut files = FileListBuilder::default();
        for item in self.items {
            Self::process_item(item, &mut files).await?;
        }

        Ok(files.build())
    }

    fn process_item<'a>(
        item: Box<dyn DynamicItem>,
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
                Self::process_item(question, files).await?;
            }

            Ok(())
        })
    }
}

impl Item for Geenie {
    fn process<'a>(self, ctx: Context<'a>) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            for item in self.items {
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
