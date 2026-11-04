use crate::multiple_responses_builder::*;
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
            CliCommand::GetThing => Self::cli_get_thing(),
            CliCommand::PutThing => Self::cli_put_thing(),
            CliCommand::GetThingContents => Self::cli_get_thing_contents(),
        }
    }

    pub fn cli_get_thing() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::std::string::String))
                .required(true),
        )
    }

    pub fn cli_put_thing() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("id")
                    .long("id")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(false)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
    }

    pub fn cli_get_thing_contents() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("id")
                .long("id")
                .value_parser(::clap::value_parser!(::std::string::String))
                .required(true),
        )
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::GetThing => self.execute_get_thing(matches).await,
            CliCommand::PutThing => self.execute_put_thing(matches).await,
            CliCommand::GetThingContents => self.execute_get_thing_contents(matches).await,
        }
    }

    pub async fn execute_get_thing(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.get_thing();
        if let Some(value) = matches.get_one::<::std::string::String>("id") {
            request = request.id(value.clone());
        }

        self.config.execute_get_thing(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                todo!()
            }
            Err(r) => {
                todo!()
            }
        }
    }

    pub async fn execute_put_thing(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.put_thing();
        if let Some(value) = matches.get_one::<::std::string::String>("id") {
            request = request.id(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value)
                .with_context(|| format!("failed to read {}", value.display()))?;
            let body_value = serde_json::from_str::<types::Thing>(&body_txt)
                .with_context(|| format!("failed to parse {}", value.display()))?;
            request = request.body(body_value);
        }

        self.config.execute_put_thing(matches, &mut request)?;
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

    pub async fn execute_get_thing_contents(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.get_thing_contents();
        if let Some(value) = matches.get_one::<::std::string::String>("id") {
            request = request.id(value.clone());
        }

        self.config
            .execute_get_thing_contents(matches, &mut request)?;
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
    fn execute_get_thing(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetThing,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_put_thing(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::PutThing,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_get_thing_contents(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GetThingContents,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    GetThing,
    PutThing,
    GetThingContents,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::GetThing,
            CliCommand::PutThing,
            CliCommand::GetThingContents,
        ]
        .into_iter()
    }

    pub fn operation_id(&self) -> &'static str {
        match self {
            CliCommand::GetThing => "get_thing",
            CliCommand::PutThing => "put_thing",
            CliCommand::GetThingContents => "get_thing_contents",
        }
    }
}
