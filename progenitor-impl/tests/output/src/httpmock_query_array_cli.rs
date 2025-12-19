use crate::httpmock_query_array_builder::*;
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
            CliCommand::ListWidgets => Self::cli_list_widgets(),
        }
    }

    pub fn cli_list_widgets() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("tags")
                .long("tags")
                .value_parser(::clap::value_parser!(
                    ::std::vec::Vec<::std::string::String>
                ))
                .required(true),
        )
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::ListWidgets => self.execute_list_widgets(matches).await,
        }
    }

    pub async fn execute_list_widgets(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.list_widgets();
        if let Some(value) = matches.get_one::<::std::vec::Vec<::std::string::String>>("tags") {
            request = request.tags(value.clone());
        }

        self.config.execute_list_widgets(matches, &mut request)?;
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
    fn execute_list_widgets(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListWidgets,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    ListWidgets,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::ListWidgets].into_iter()
    }
}
