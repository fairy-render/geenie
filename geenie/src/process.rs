use std::path::Path;

use async_process::Command;

use crate::{Environment, GeenieError, Item};

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

impl<E: Environment> crate::command::Command<E> for Process {
    fn run<'a>(
        &'a self,
        env: &'a E,
        path: &'a Path,
    ) -> impl std::future::Future<Output = Result<(), GeenieError>> + 'a {
        async move {
            let cmd = format!("{} {}", self.cmd, self.args.join(" "));

            let o = env
                .work(&format!("Executing {}", cmd), async move {
                    let ret = Command::new(&self.cmd)
                        .args(&self.args)
                        .current_dir(path)
                        .output()
                        .await?;

                    Ok((format!("Executed {}", cmd), ret))
                })
                .await?;

            if self.output {
                env.info(&*String::from_utf8_lossy(&o.stdout))
                    .await
                    .map_err(GeenieError::backend)?;
            }

            if !o.status.success() {
                return Err(GeenieError::command(
                    String::from_utf8_lossy(&o.stderr).to_string(),
                ));
            }

            Ok(())
        }
    }
}

impl<E: Environment, C> Item<E, C> for Process {
    fn process<'a>(
        self,
        mut ctx: crate::Context<'a, E, C>,
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
