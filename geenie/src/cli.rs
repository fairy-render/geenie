use blocking::unblock;

use crate::Environment;

#[derive(Default, Clone, Copy)]
pub struct Cli;

impl Environment for Cli {
    type Error = std::io::Error;

    fn info(&self, message: &str) -> impl std::future::Future<Output = Result<(), Self::Error>> {
        let message = message.to_string();
        async move { unblock(|| cliclack::log::info(message)).await }
    }

    fn error(&self, message: &str) -> impl std::future::Future<Output = Result<(), Self::Error>> {
        let message = message.to_string();
        async move { unblock(|| cliclack::log::error(message)).await }
    }

    fn confirm(
        &self,
        confirm: crate::questions::Confirm,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> {
        async move {
            let ret = unblock(move || {
                cliclack::confirm(confirm.label)
                    .initial_value(confirm.default)
                    .interact()
            })
            .await?;
            Ok(ret)
        }
    }

    fn input(
        &self,
        input: crate::questions::Input,
    ) -> impl std::future::Future<Output = Result<String, Self::Error>> {
        async move {
            let ret = unblock(move || cliclack::input(input.label).interact()).await?;
            Ok(ret)
        }
    }

    fn select<T: Send + Clone + Eq + 'static>(
        &self,
        input: crate::questions::Select<T>,
    ) -> impl std::future::Future<Output = Result<T, Self::Error>> {
        async move {
            let ret = unblock(move || {
                cliclack::select::<T>(input.label)
                    .items(&*input.items)
                    .interact()
            })
            .await?;
            Ok(ret)
        }
    }

    fn multiselect<T: Send + Clone + Eq + 'static>(
        &self,
        input: crate::questions::MultiSelect<T>,
    ) -> impl std::future::Future<Output = Result<Vec<T>, Self::Error>> {
        async move {
            let ret = unblock(move || {
                cliclack::multiselect::<T>(input.label)
                    .items(&*input.items)
                    .interact()
            })
            .await?;
            Ok(ret)
        }
    }
}
