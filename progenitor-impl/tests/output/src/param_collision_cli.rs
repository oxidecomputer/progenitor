use crate::param_collision_builder::*;
pub struct Cli<T: CliConfig> {
    client: Client,
    config: T,
}

impl<T: CliConfig> Cli<T> {
    pub fn new(client: Client, config: T) -> Self {
        Self { client, config }
    }

    pub fn get_command(cmd: CliCommand) -> clap::Command {
        match cmd {
            CliCommand::KeyGet => Self::cli_key_get(),
        }
    }

    pub fn cli_key_get() -> clap::Command {
        clap::Command::new("")
            .arg(
                clap::Arg::new("client")
                    .long("client")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .arg(
                clap::Arg::new("query")
                    .long("query")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .arg(
                clap::Arg::new("request")
                    .long("request")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .arg(
                clap::Arg::new("response")
                    .long("response")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .arg(
                clap::Arg::new("result")
                    .long("result")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .arg(
                clap::Arg::new("url")
                    .long("url")
                    .value_parser(clap::value_parser!(bool))
                    .required(true)
                    .help("Parameter name that was previously colliding"),
            )
            .long_about("Gets a key")
            .display_order(0)
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        match cmd {
            CliCommand::KeyGet => self.execute_key_get(matches).await,
        }
    }

    pub async fn execute_key_get(&self, matches: &clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.key_get();
        if let Some(value) = matches.get_one::<bool>("client") {
            request = request.client(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("query") {
            request = request.query(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("request") {
            request = request.request(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("response") {
            request = request.response(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("result") {
            request = request.result(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("url") {
            request = request.url(value.clone());
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
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn success_no_item(&self, value: &ResponseValue<()>);
    fn error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_start<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_item<T>(&self, value: &T)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_success<T>(&self)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn list_end_error<T>(&self, value: &Error<T>)
    where
        T: schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_key_get(
        &self,
        matches: &clap::ArgMatches,
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
