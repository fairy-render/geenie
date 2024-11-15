mod command;
mod context;
mod error;
mod file;
mod func;
mod geenie;
mod item;
#[cfg(feature = "process")]
mod process;
// mod question;
mod result;

mod machine;

#[cfg(feature = "cli")]
pub mod cli;

pub use self::{
    command::{Command, DynamicCommand},
    context::Context,
    error::GeenieError,
    file::{File, FileList},
    geenie::Geenie,
    item::{Item, ItemExt, MountItem},
    // question::{Question, QuestionKind, QuestionKindExt},
    machine::{questions, Environment, Question, QuestionKind, QuestionKindExt},
};

#[cfg(feature = "process")]
pub use self::process::*;

#[cfg(feature = "cli")]
pub use cliclack;
