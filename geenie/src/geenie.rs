use core::{future::Future, pin::Pin};

use crate::{
    command::{Command, CommandItem},
    item::{DynamicItem, ItemBox},
    machine::{Question, QuestionBox},
    result::{GeenieResult, ResultBuilder},
    Context, GeenieError, Item,
};

#[derive(Default)]
pub struct Geenie<E, C> {
    env: E,
    items: Vec<Box<dyn DynamicItem<E, C>>>,
}

impl<E, C> Geenie<E, C> {
    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item<E, C> + 'static,
    {
        self.items.push(Box::new(ItemBox(item)));
        self
    }

    pub fn ask<T>(&mut self, question: T) -> &mut Self
    where
        T: Question<E, C> + 'static,
    {
        self.items.push(Box::new(QuestionBox(question)));
        self
    }

    pub fn command<T>(&mut self, command: T) -> &mut Self
    where
        T: Command<E> + 'static,
    {
        self.push(CommandItem(command));
        self
    }

    pub async fn run(self, context: &mut C) -> Result<GeenieResult<E>, GeenieError> {
        let mut files = ResultBuilder::<E>::default();
        for item in self.items {
            Self::process_item(&self.env, item, &mut files, context).await?;
        }

        Ok(files.build(self.env))
    }

    fn process_item<'a>(
        env: &'a E,
        item: Box<dyn DynamicItem<E, C>>,
        files: &'a mut ResultBuilder<E>,
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
                env: env,
            })
            .await?;

            for question in questions {
                Self::process_item(env, question, files, context).await?;
            }

            Ok(())
        })
    }
}

impl<E, C> Item<E, C> for Geenie<E, C> {
    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            for item in self.items {
                item.process(Context {
                    files: ctx.files,
                    questions: ctx.questions,
                    ctx: ctx.ctx,
                    env: ctx.env,
                })
                .await?;
            }

            Ok(())
        }
    }
}
