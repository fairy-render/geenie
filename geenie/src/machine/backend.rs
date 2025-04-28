use std::future::Future;

use crate::GeenieError;

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

    fn work<T, O>(&self, message: &str, future: T) -> impl Future<Output = Result<O, GeenieError>>
    where
        T: Future<Output = Result<(String, O), GeenieError>>;
}
