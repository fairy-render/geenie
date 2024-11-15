use std::future::Future;

use super::questions::{Confirm, Input, MultiSelect, Select};

pub trait Environment {
    type Error: std::error::Error + Send + Sync + 'static;
    fn info(&self, message: &str) -> impl Future<Output = Result<(), Self::Error>>;
    fn error(&self, error: &str) -> impl Future<Output = Result<(), Self::Error>>;

    fn confirm(&self, confirm: Confirm) -> impl Future<Output = Result<bool, Self::Error>>;
    fn input(&self, input: Input) -> impl Future<Output = Result<String, Self::Error>>;
    fn select<T: Send + Clone + Eq + 'static>(
        &self,
        input: Select<T>,
    ) -> impl Future<Output = Result<T, Self::Error>>;
    fn multiselect<T: Send + Clone + Eq + 'static>(
        &self,
        input: MultiSelect<T>,
    ) -> impl Future<Output = Result<Vec<T>, Self::Error>>;
}

// pub trait QuestionKind<E: Environment> {
//     type Output;

//     fn run(self, env: &E) -> impl Future<Output = Result<Self::Output, <E as Environment>::Error>>;
// }

// pub trait Question<E: Environment, C> {
//     type Kind: QuestionKind<E>;

//     fn crate_question(&self) -> Result<Self::Kind, GeenieError>;

//     fn process<'a>(
//         self,
//         ctx: Context<'a, E, C>,
//         answer: <Self::Kind as QuestionKind<E>>::Output,
//     ) -> impl Future<Output = Result<(), GeenieError>> + 'a;
// }