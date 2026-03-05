use crate::websocket_default_builder::*;
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
            CliCommand::WebsocketWithoutExplicit101 => Self::cli_websocket_without_explicit_101(),
        }
    }

    pub fn cli_websocket_without_explicit_101() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::WebsocketWithoutExplicit101 => {
                self.execute_websocket_without_explicit_101(matches).await
            }
        }
    }

    pub async fn execute_websocket_without_explicit_101(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.websocket_without_explicit_101();
        self.config
            .execute_websocket_without_explicit_101(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
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
    fn execute_websocket_without_explicit_101(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WebsocketWithoutExplicit101,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    WebsocketWithoutExplicit101,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::WebsocketWithoutExplicit101].into_iter()
    }
}
