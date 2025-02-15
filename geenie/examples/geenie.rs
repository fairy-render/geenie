use core::fmt;

use geenie::{
    cli::Cli,
    process,
    questions::{confirm, input, select},
    Context, Environment, File, Geenie, GeenieError, Item, ItemExt, QuestionKindExt,
};
use relative_path::RelativePathBuf;

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

impl<E: Environment + 'static, C: 'static> Item<E, C> for Test {
    fn process<'a>(
        self,
        mut ctx: geenie::Context<'a, E, C>,
    ) -> impl std::future::Future<Output = Result<(), geenie::GeenieError>> + 'a {
        async move {
            // ctx.file(File {
            //     path: RelativePathBuf::from("package.json"),
            //     content: b"{}".to_vec(),
            // })?;

            // ctx.ask(input("Name").question(|mut ctx: Context<'_, E, C>, ans| {
            //     ctx.file(File {
            //         path: RelativePathBuf::from(format!("{ans}.json")),
            //         content: b"{}".to_vec(),
            //     })?;
            //     Ok(())
            // }));

            // ctx.ask(
            //     (
            //         select("Bundler").item(Bundler::Vite, "Vite", "").item(
            //             Bundler::Webpack,
            //             "Webpack",
            //             "",
            //         ),
            //         confirm("Typescript").default(true),
            //     )
            //         .question(
            //             |mut ctx: Context<'_, E, C>, ans: (Bundler, bool)| {
            //                 ctx.file(File::new(
            //                     "inner/info.json",
            //                     format!(r#"{{"bundler":"{:?}", "typescript": {}}}"#, ans.0, ans.1),
            //                 ))?;
            //                 Ok(())
            //             },
            //         ),
            // );

            ctx.command(process("pnpm").arg("-h").output(true));

            Ok(())
        }
    }
}

fn main() -> Result<(), GeenieError> {
    futures::executor::block_on(async move {
        ctrlc::set_handler(move || {}).expect("setting Ctrl-C handler");
        let mut m = Geenie::<Cli, ()>::default();

        m.push(<Test as ItemExt<Cli, ()>>::mount(Test, "subpath"));

        let files = m.run(&mut ()).await?;

        let spinner = cliclack::spinner();
        spinner.start("Creating files");
        files.write_to("geenie-test", false).await?;
        std::thread::sleep(std::time::Duration::from_secs(1));
        spinner.stop("Files created");

        Result::<_, GeenieError>::Ok(())
    })
}
