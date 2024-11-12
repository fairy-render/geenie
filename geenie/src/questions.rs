use core::fmt::Display;

use crate::QuestionKind;
use blocking::unblock;

pub struct Input {
    label: String,
}

impl QuestionKind for Input {
    type Output = String;
    fn ask(self) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> {
        async move {
            let ret = unblock(move || cliclack::input(self.label).interact()).await?;
            Ok(ret)
        }
    }
}

pub fn input(input: impl Display) -> Input {
    Input {
        label: input.to_string(),
    }
}

pub struct Confirm {
    label: String,
    default: bool,
}

impl Confirm {
    pub fn initial_value(mut self, value: bool) -> Confirm {
        self.default = value;
        self
    }
}

impl QuestionKind for Confirm {
    type Output = bool;
    fn ask(self) -> impl std::future::Future<Output = Result<Self::Output, crate::GeenieError>> {
        async move {
            let ret = unblock(move || {
                cliclack::confirm(self.label)
                    .initial_value(self.default)
                    .interact()
            })
            .await?;
            Ok(ret)
        }
    }
}

pub fn confirm(input: impl Display) -> Confirm {
    Confirm {
        label: input.to_string(),
        default: false,
    }
}

pub struct Select<T> {
    label: String,
    items: Vec<(T, String, String)>,
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

impl<T: Display + Send + Clone + Eq + 'static> QuestionKind for Select<T> {
    type Output = T;

    async fn ask(self) -> Result<Self::Output, crate::GeenieError> {
        let ret = unblock(move || {
            cliclack::select::<T>(self.label)
                .items(&*self.items)
                .interact()
        })
        .await?;
        Ok(ret)
    }
}

pub fn select<T>(input: impl Display) -> Select<T> {
    Select {
        label: input.to_string(),
        items: Vec::default(),
    }
}

pub struct MultiSelect<T> {
    label: String,
    items: Vec<(T, String, String)>,
}

impl<T: Display + Send + Clone + Eq + 'static> QuestionKind for MultiSelect<T> {
    type Output = Vec<T>;

    async fn ask(self) -> Result<Self::Output, crate::GeenieError> {
        let ret = unblock(move || {
            cliclack::multiselect::<T>(self.label)
                .items(&*self.items)
                .interact()
        })
        .await?;
        Ok(ret)
    }
}

pub fn multiselect<T>(input: impl Display) -> MultiSelect<T> {
    MultiSelect {
        label: input.to_string(),
        items: Vec::default(),
    }
}
