use std::path::Path;

use async_process::Command;

use crate::{GeenieError, Item};

pub struct Process {
    cmd: String,
    args: Vec<String>,
    output: bool,
}

impl Process {
    pub fn arg(mut self, arg: impl ToString) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn output(mut self, output: bool) -> Self {
        self.output = output;
        self
    }
}

impl crate::command::Command for Process {
    fn run<'a>(
        &'a self,
        path: &'a Path,
    ) -> impl std::future::Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let o = Command::new(&self.cmd)
                .args(&self.args)
                .current_dir(path)
                .output()
                .await?;

            if !o.status.success() {
                return Err(GeenieError::command(
                    String::from_utf8_lossy(&o.stderr).to_string(),
                ));
            }

            if self.output {
                println!("{}", String::from_utf8_lossy(&o.stdout),)
            }

            Ok(())
        }
    }
}

impl<C> Item<C> for Process {
    fn process<'a>(
        self,
        mut ctx: crate::Context<'a, C>,
    ) -> impl std::future::Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            ctx.command(self);
            Ok(())
        }
    }
}

pub fn process(cmd: impl ToString) -> Process {
    Process {
        cmd: cmd.to_string(),
        args: Vec::new(),
        output: false,
    }
}
