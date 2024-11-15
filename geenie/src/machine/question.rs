use core::{cell::RefCell, future::Future, pin::Pin};

use crate::{func::Func, item::DynamicItem, Context, GeenieError, Item};

pub trait QuestionKind<E> {
    type Output;

    fn ask<'a>(self, env: &'a E) -> impl Future<Output = Result<Self::Output, GeenieError>> + 'a;
}

macro_rules! impl_questions {
    ($first: ident) => {
        impl<E, $first: QuestionKind<E> + 'static> QuestionKind<E> for ($first,) {
            type Output = ($first::Output,);

            fn ask<'a>(self, env: &'a E) -> impl Future<Output = Result<Self::Output, GeenieError>> +'a {
                async move { Ok((self.0.ask(env).await?,)) }
            }
        }
    };

    ($first: ident, $($rest: ident),*) => {
        impl_questions!($($rest),*);

        impl<E, $first: QuestionKind<E> + 'static, $($rest: QuestionKind<E> + 'static),*> QuestionKind<E> for ($first, $($rest),*) {
            type Output = ($first::Output, $($rest::Output),*);

            #[allow(non_snake_case)]
            fn ask<'a>(self, env: &'a E) -> impl Future<Output = Result<Self::Output, GeenieError>> {
                async move {

                    let ($first,$($rest),*) = self;

                    Ok((
                        $first.ask(env).await?,
                        $(
                            $rest.ask(env).await?
                        ),*
                    ))

                }
            }

        }
    };
}

impl_questions!(T1, T2, T3, T4, T5, T6, T7, T8);

pub trait Question<E, C> {
    type Kind: QuestionKind<E>;

    fn crate_question(&self) -> Result<Self::Kind, GeenieError>;

    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
        answer: <Self::Kind as QuestionKind<E>>::Output,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a;
}

pub struct QuestionBox<T>(pub T);

impl<T, E, C> DynamicItem<E, C> for QuestionBox<T>
where
    T: Question<E, C> + 'static,
{
    fn process<'a>(
        self: Box<Self>,
        ctx: Context<'a, E, C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), GeenieError>> + 'a>> {
        Box::pin(async move {
            let question = self.0.crate_question()?;
            let answer = question.ask(&ctx.env).await?;
            self.0.process(ctx, answer).await?;
            Ok(())
        })
    }
}

pub trait QuestionKindExt<E>: QuestionKind<E> {
    fn question<C, T>(self, func: T) -> SimpleQuestion<T, Self>
    where
        Self: Sized,
        for<'a> T: Func<'a, E, C, Self::Output>,
    {
        SimpleQuestion {
            func,
            kind: RefCell::new(Some(self)),
        }
    }
}

impl<E, T> QuestionKindExt<E> for T where T: QuestionKind<E> {}

pub struct SimpleQuestion<T, K> {
    func: T,
    kind: RefCell<Option<K>>,
}

impl<T, K, E, C> Question<E, C> for SimpleQuestion<T, K>
where
    K: QuestionKind<E>,
    K::Output: 'static,
    T: 'static,
    for<'a> T: FnOnce(Context<'a, E, C>, K::Output) -> Result<(), GeenieError>,
{
    type Kind = K;

    fn crate_question(&self) -> Result<Self::Kind, GeenieError> {
        let Some(kind) = self.kind.take() else {
            panic!("create question")
        };
        Ok(kind)
    }

    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
        answer: <Self::Kind as QuestionKind<E>>::Output,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            (self.func)(ctx, answer)?;
            Ok(())
        }
    }
}

impl<T, K, E, C> Item<E, C> for SimpleQuestion<T, K>
where
    K: QuestionKind<E> + 'static,
    K::Output: 'static,
    T: 'static,
    for<'a> T: FnOnce(Context<'a, E, C>, K::Output) -> Result<(), GeenieError>,
{
    fn process<'a>(
        self,
        ctx: Context<'a, E, C>,
    ) -> impl Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let question = self.crate_question()?;
            let answer = question.ask(ctx.env).await?;
            (self.func)(ctx, answer)?;
            Ok(())
        }
    }
}
