use crate::{
    command::{Command, CommandBox},
    item::{DynamicItem, ItemBox},
    question::QuestionBox,
    result::ResultBuilder,
    File, GeenieError, Item, Question,
};

pub struct Context<'a, C> {
    pub(crate) files: &'a mut ResultBuilder,
    pub(crate) questions: &'a mut Vec<Box<dyn DynamicItem<C>>>,
    pub(crate) ctx: &'a mut C,
}

impl<'a, C> Context<'a, C> {
    pub fn ask<T: Question<C> + 'static>(&mut self, question: T) -> &mut Self {
        self.questions.push(Box::new(QuestionBox(question)));
        self
    }

    pub fn push<T>(&mut self, item: T) -> &mut Self
    where
        T: Item<C> + 'static,
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
        T: Command + 'static,
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
