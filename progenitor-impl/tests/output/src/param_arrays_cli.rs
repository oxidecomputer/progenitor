use crate::param_arrays_builder::*;
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
            CliCommand::ListThings => Self::cli_list_things(),
        }
    }

    pub fn cli_list_things() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("kinds")
                    .long("kinds")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::ThingKind::Widget.to_string(),
                            types::ThingKind::Gadget.to_string(),
                        ]),
                        |s| types::ThingKind::try_from(s).unwrap(),
                    ))
                    .action(::clap::ArgAction::Append)
                    .required(true)
                    .help("kinds of thing to include"),
            )
            .arg(
                ::clap::Arg::new("tags")
                    .long("tags")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .action(::clap::ArgAction::Append)
                    .required(false)
                    .help("tags that each thing must have"),
            )
            .arg(
                ::clap::Arg::new("x-labels")
                    .long("x-labels")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .action(::clap::ArgAction::Append)
                    .required(false)
                    .help("labels for the request"),
            )
            .arg(
                ::clap::Arg::new("x-shard-ids")
                    .long("x-shard-ids")
                    .value_parser(::clap::value_parser!(u32))
                    .action(::clap::ArgAction::Append)
                    .required(true)
                    .help("shards to read from"),
            )
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::ListThings => self.execute_list_things(matches).await,
        }
    }

    pub async fn execute_list_things(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.list_things();
        if let Some(values) = matches.get_many::<types::ThingKind>("kinds") {
            request = request.kinds(
                values
                    .cloned()
                    .collect::<::std::vec::Vec<types::ThingKind>>(),
            );
        }

        if let Some(values) = matches.get_many::<::std::string::String>("tags") {
            request = request.tags(
                values
                    .cloned()
                    .collect::<::std::vec::Vec<::std::string::String>>(),
            );
        }

        if let Some(values) = matches.get_many::<::std::string::String>("x-labels") {
            request = request.x_labels(
                values
                    .cloned()
                    .collect::<::std::vec::Vec<::std::string::String>>(),
            );
        }

        if let Some(values) = matches.get_many::<u32>("x-shard-ids") {
            request = request.x_shard_ids(values.cloned().collect::<::std::vec::Vec<u32>>());
        }

        self.config.execute_list_things(matches, &mut request)?;
        let result = request.send().await;
        match result {
            Ok(r) => {
                self.config.success_item(&r);
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
    fn json_body_schema<T>(&self)
    where
        T: std::clone::Clone + schemars::JsonSchema + serde::Serialize + std::fmt::Debug;
    fn execute_list_things(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ListThings,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    ListThings,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::ListThings].into_iter()
    }

    pub fn operation_id(&self) -> &'static str {
        match self {
            CliCommand::ListThings => "list_things",
        }
    }
}
