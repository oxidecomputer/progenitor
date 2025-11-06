use crate::generated_operation_id_builder::*;
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
            CliCommand::Get => Self::cli_get(),
            CliCommand::DoPing => Self::cli_do_ping(),
        }
    }

    pub fn cli_get() -> ::clap::Command {
        ::clap::Command::new("").long_about("Ping")
    }

    pub fn cli_do_ping() -> ::clap::Command {
        ::clap::Command::new("").long_about("Ping")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::Get => self.execute_get(matches).await,
            CliCommand::DoPing => self.execute_do_ping(matches).await,
        }
    }

    pub async fn execute_get(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get();
        self.config.execute_get(matches, &mut request)?;
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

    pub async fn execute_do_ping(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.do_ping();
        self.config.execute_do_ping(matches, &mut request)?;
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
    fn execute_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::Get,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_do_ping(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::DoPing,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    Get,
    DoPing,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::Get, CliCommand::DoPing].into_iter()
    }
}
