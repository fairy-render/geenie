use std::{cell::RefCell, future::Future, pin::Pin};

use crate::{file::FileListBuilder, func::Func, item::DynamicItem, File, GeenieError, Item};

pub trait QuestionKind {
    type Output;

    fn ask(self) -> impl Future<Output = Result<Self::Output, GeenieError>>;
}

macro_rules! impl_questions {
    ($first: ident) => {
        impl<$first: QuestionKind> QuestionKind for ($first,) {
            type Output = ($first::Output,);

            fn ask(self) -> impl Future<Output = Result<Self::Output, GeenieError>> {
                async move { Ok((self.0.ask().await?,)) }
            }
        }
    };

    ($first: ident, $($rest: ident),*) => {
        impl_questions!($($rest),*);

        impl<$first: QuestionKind, $($rest: QuestionKind),*> QuestionKind for ($first, $($rest),*) {
            type Output = ($first::Output, $($rest::Output),*);

            #[allow(non_snake_case)]
            fn ask(self) -> impl Future<Output = Result<Self::Output, GeenieError>> {
                async move {

                    let ($first,$($rest),*) = self;

                    Ok((
                        $first.ask().await?,
                        $(
                            $rest.ask().await?
                        ),*
                    ))

                }
            }

        }
    };
}

impl_questions!(T1, T2, T3, T4, T5, T6, T7, T8);

pub trait Question {
    type Kind: QuestionKind;

    fn crate_question(&self) -> Result<Self::Kind, GeenieError>;

    fn process<'a>(
        self,
        ctx: Context<'a>,
        answer: <Self::Kind as QuestionKind>::Output,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

pub trait DynamicQuestion {
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>>;
}

pub struct QuestionBox<T>(pub T);

impl<T> DynamicQuestion for QuestionBox<T>
where
    T: Question + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move {
            let question = self.0.crate_question()?;
            let answer = question.ask().await?;
            self.0.process(ctx, answer).await?;
            Ok(())
        })
    }
}

pub struct Context<'a> {
    pub(crate) files: &'a mut FileListBuilder,
    pub(crate) questions: &'a mut Vec<Box<dyn DynamicQuestion>>,
}

impl<'a> Context<'a> {
    pub fn ask<T: Question + 'static>(&mut self, question: T) -> &mut Self {
        self.questions.push(Box::new(QuestionBox(question)));
        self
    }

    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item + 'static,
    {
        self.questions.push(Box::new(ItemQuestion(item)));
        self
    }

    pub fn file(&mut self, file: impl Into<File>) -> Result<&mut Self, GeenieError> {
        self.files.push(file.into())?;
        Ok(self)
    }
}

pub trait QuestionKindExt: QuestionKind {
    fn question<T>(self, func: T) -> SimpleQuestion<T, Self>
    where
        Self: Sized,
        for<'a> T: Func<'a, Self::Output>,
    {
        SimpleQuestion {
            func,
            kind: RefCell::new(Some(self)),
        }
    }
}

impl<T> QuestionKindExt for T where T: QuestionKind {}

pub struct SimpleQuestion<T, K> {
    func: T,
    kind: RefCell<Option<K>>,
}

impl<T, K> Question for SimpleQuestion<T, K>
where
    K: QuestionKind,
    K::Output: 'static,
    T: 'static,
    for<'a> T: FnOnce(Context<'a>, K::Output) -> Result<(), GeenieError>,
{
    type Kind = K;

    fn crate_question(&self) -> Result<Self::Kind, GeenieError> {
        let Some(kind) = self.kind.take() else {
            todo!("ewrror")
        };
        Ok(kind)
    }

    fn process<'a>(
        self,
        ctx: Context<'a>,
        answer: <Self::Kind as QuestionKind>::Output,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            (self.func)(ctx, answer)?;
            Ok(())
        }
    }
}

impl<T, K> Item for SimpleQuestion<T, K>
where
    K: QuestionKind + 'static,
    K::Output: 'static,
    T: 'static,
    for<'a> T: FnOnce(Context<'a>, K::Output) -> Result<(), GeenieError>,
{
    fn process<'a>(self, ctx: Context<'a>) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let question = self.crate_question()?;
            let answer = question.ask().await?;
            (self.func)(ctx, answer)?;
            Ok(())
        }
    }
}

pub struct ItemQuestion<T>(pub T);

impl<T> DynamicQuestion for ItemQuestion<T>
where
    T: Item + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move {
            self.0.process(ctx).await?;
            Ok(())
        })
    }
}
