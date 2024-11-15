use crate::{
    command::{Command, CommandBox},
    item::{DynamicItem, ItemBox},
    machine::{Question, QuestionBox},
    result::ResultBuilder,
    File, GeenieError, Item,
};

pub struct Context<'a, E, C> {
    pub(crate) files: &'a mut ResultBuilder<E>,
    pub(crate) questions: &'a mut Vec<Box<dyn DynamicItem<E, C>>>,
    pub(crate) ctx: &'a mut C,
    pub(crate) env: &'a E,
}

impl<'a, E, C> Context<'a, E, C> {
    pub fn ask<T: Question<E, C> + 'static>(&mut self, question: T) -> &mut Self {
        self.questions.push(Box::new(QuestionBox(question)));
        self
    }

    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item<E, C> + 'static,
    {
        self.questions.push(Box::new(ItemBox(item)));
        self
    }

    pub fn file(&mut self, file: impl Into<File>) -> Result<&mut Self, GeenieError> {
        self.files.push_file(file.into())?;
        Ok(self)
    }

    pub fn command<T>(&mut self, command: T) -> &mut Self
    where
        T: Command<E> + 'static,
    {
        self.files.push_command(Box::new(CommandBox(command)));
        self
    }

    pub fn data_mut(&mut self) -> &mut C {
        self.ctx
    }

    pub fn data(&self) -> &C {
        self.ctx
    }
}
