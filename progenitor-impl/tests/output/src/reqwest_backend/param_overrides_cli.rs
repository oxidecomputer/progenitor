use crate::param_overrides_builder::*;
use anyhow::Context as _;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}

impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }

    pub fn get_command(cmd: CliCommand) -> ::clap::Command {
        match cmd {
            CliCommand::KeyGet => Self::cli_key_get(),
        }
    }

    pub fn cli_key_get() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("key")
                    .long("key")
                    .value_parser(::clap::value_parser!(bool))
                    .required(false)
                    .help("The same key parameter that overlaps with the path level parameter"),
            )
            .arg(
                ::clap::Arg::new("unique-key")
                    .long("unique-key")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("A key parameter that will not be overridden by the path spec"),
            )
            .long_about("Gets a key")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::KeyGet => self.execute_key_get(matches).await,
        }
    }

    pub async fn execute_key_get(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.key_get();
        if let Some(value) = matches.get_one::<bool>("key") {
            request = request.key(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("unique-key") {
            request = request.unique_key(value.clone());
        }

        self.config.execute_key_get(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_no_item(&r);
                Ok(())
            }
            Err(r) => {
                self.config.error(&r);
                Err(anyhow::Error::new(r))
            }
        }
    }
}

pub trait CliConfig {
    fn success_item<T>(&self, value: &ResponseValue<T>)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn success_no_item(&self, value: &ResponseValue<()>);
    fn error<T>(&self, value: &Error<T>)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_key_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::KeyGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    KeyGet,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::KeyGet].into_iter()
    }
}
