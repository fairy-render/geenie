use core::fmt;

use geenie::{
    process,
    questions::{confirm, input, select},
    Cli, Context, File, Geenie, GeenieError, Item, ItemExt,
};
use relative_path::RelativePathBuf;
use spurgt::{core::Env, Asger, Spurgt};

struct Test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bundler {
    Vite,
    Webpack,
}

impl fmt::Display for Bundler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<E: Asger + 'static, C: 'static> Item<E, C> for Test {
    fn process<'a>(
        self,
        mut ctx: geenie::Context<'a, E, C>,
        env: &'a mut Spurgt<E>,
    ) -> impl std::future::Future<Output = Result<(), geenie::GeenieError>> + 'a {
        async move {
            // ctx.file(File {
            //     path: RelativePathBuf::from("package.json"),
            //     content: b"{}".to_vec(),
            // })?;

            env.ask(input("Hello, World")).await?;

            ctx.command(process("pnpm").arg("-h").output(false));

            Ok(())
        }
    }
}

fn main() -> Result<(), GeenieError> {
    futures::executor::block_on(async move {
        ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");
        let mut m = Geenie::<Cli, ()>::default();

        m.push(Test);

        let mut files = m.run(&mut ()).await?;

        files.write_to("geenie-test", false).await?;

        Result::<_, GeenieError>::Ok(())
    })
}
