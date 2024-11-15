use core::{future::Future, pin::Pin};

use crate::{
    command::{Command, CommandItem},
    item::{DynamicItem, ItemBox},
    question::QuestionBox,
    result::{GeenieResult, ResultBuilder},
    Context, GeenieError, Item, Question,
};

#[derive(Default)]
pub struct Geenie<C> {
    items: Vec<Box<dyn DynamicItem<C>>>,
}

impl<C> Geenie<C> {
    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item<C> + 'static,
    {
        self.items.push(Box::new(ItemBox(item)));
        self
    }

    pub fn ask<T>(&mut self, question: T) -> &mut Self
    where
        T: Question<C> + 'static,
    {
        self.items.push(Box::new(QuestionBox(question)));
        self
    }

    pub fn command<T>(&mut self, command: T) -> &mut Self
    where
        T: Command + 'static,
    {
        self.push(CommandItem(command));
        self
    }

    pub async fn run(self, context: &mut C) -> Result<GeenieResult, GeenieError> {
        let mut files = ResultBuilder::default();
        for item in self.items {
            Self::process_item(item, &mut files, context).await?;
        }

        Ok(files.build())
    }

    fn process_item<'a>(
        item: Box<dyn DynamicItem<C>>,
        files: &'a mut ResultBuilder,
        context: &'a mut C,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>
    where
        C: 'a,
    {
        Box::pin(async move {
            let mut questions = Vec::default();

            item.process(Context {
                files,
                questions: &mut questions,
                ctx: context,
            })
            .await?;

            for question in questions {
                Self::process_item(question, files, context).await?;
            }

            Ok(())
        })
    }
}

impl<C> Item<C> for Geenie<C> {
    fn process<'a>(
        self,
        ctx: Context<'a, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            for item in self.items {
                item.process(Context {
                    files: ctx.files,
                    questions: ctx.questions,
                    ctx: ctx.ctx,
                })
                .await?;
            }

            Ok(())
        }
    }
}
