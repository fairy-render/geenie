use std::fmt::Display;

use crate::GeenieError;

use super::{Environment, QuestionKind};

pub fn confirm(input: impl Display) -> Confirm {
    Confirm {
        label: input.to_string(),
        default: false,
    }
}

pub fn select<T>(input: impl Display) -> Select<T> {
    Select {
        label: input.to_string(),
        items: Vec::default(),
    }
}

pub fn multiselect<T>(input: impl Display) -> MultiSelect<T> {
    MultiSelect {
        label: input.to_string(),
        items: Vec::default(),
    }
}

pub fn input(input: impl Display) -> Input {
    Input {
        label: input.to_string(),
        default: None,
        required: false,
    }
}

pub struct Confirm {
    pub label: String,
    pub default: bool,
}

impl Confirm {
    pub fn new(label: impl ToString) -> Confirm {
        Confirm {
            label: label.to_string(),
            default: false,
        }
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = value;
        self
    }
}

impl<E: Environment> QuestionKind<E> for Confirm
where
    E::Error: std::error::Error + Send + Sync + 'static,
{
    type Output = bool;
    fn ask<'a>(
        self,
        env: &'a E,
    ) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> + 'a {
        async move { env.confirm(self).await.map_err(GeenieError::backend) }
    }
}

pub struct Input {
    pub label: String,
    pub default: Option<String>,
    pub required: bool,
}

impl<E: Environment> QuestionKind<E> for Input
where
    E::Error: std::error::Error + Send + Sync + 'static,
{
    type Output = String;
    fn ask<'a>(
        self,
        env: &'a E,
    ) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> + 'a {
        async move { env.input(self).await.map_err(GeenieError::backend) }
    }
}

pub struct Select<T> {
    pub label: String,
    pub items: Vec<(T, String, String)>,
}

impl<T> Select<T> {
    pub fn items<V: IntoIterator<Item = (T, String, String)>>(mut self, items: V) -> Self {
        self.items.extend(items);
        self
    }

    pub fn item(mut self, item: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push((item, label.to_string(), hint.to_string()));
        self
    }
}

impl<E: Environment, T> QuestionKind<E> for Select<T>
where
    T: Send + Clone + Eq + 'static,
    E::Error: std::error::Error + Send + Sync + 'static,
{
    type Output = T;
    fn ask<'a>(
        self,
        env: &'a E,
    ) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> + 'a {
        async move { env.select(self).await.map_err(GeenieError::backend) }
    }
}

pub struct MultiSelect<T> {
    pub label: String,
    pub items: Vec<(T, String, String)>,
}

impl<T> MultiSelect<T> {
    pub fn items<V: IntoIterator<Item = (T, String, String)>>(mut self, items: V) -> Self {
        self.items.extend(items);
        self
    }

    pub fn item(mut self, item: T, label: impl Display, hint: impl Display) -> Self {
        self.items.push((item, label.to_string(), hint.to_string()));
        self
    }
}

impl<E: Environment, T> QuestionKind<E> for MultiSelect<T>
where
    T: Send + Clone + Eq + 'static,
    E::Error: std::error::Error + Send + Sync + 'static,
{
    type Output = Vec<T>;
    fn ask<'a>(
        self,
        env: &'a E,
    ) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> + 'a {
        async move { env.multiselect(self).await.map_err(GeenieError::backend) }
    }
}
