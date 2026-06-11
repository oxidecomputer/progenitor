use crate::cli_gen_builder::*;
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
            CliCommand::Uno => Self::cli_uno(),
        }
    }

    pub fn cli_uno() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("gateway")
                    .long("gateway")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present_any(["json-body-template", "json-body-schema"]),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("Build a JSON request-body template and exit"),
            )
            .arg(
                ::clap::Arg::new("json-body-schema")
                    .long("json-body-schema")
                    .action(::clap::ArgAction::SetTrue)
                    .help("Output a grammar reference for the request body and exit"),
            )
            .group(
                ::clap::ArgGroup::new("body-source")
                    .args(["json-body", "json-body-template", "json-body-schema"])
                    .required(true)
                    .multiple(false),
            )
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::Uno => self.execute_uno(matches).await,
        }
    }

    pub async fn execute_uno(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.uno();
        if let Some(value) = matches.get_one::<::std::string::String>("gateway") {
            request = request.gateway(value.clone());
        }

        if matches.get_flag("json-body-template") {
            let schema = schemars::schema_for!(types::UnoBody);
            if let Some(body) = self.config.build_body_template(&schema)? {
                println!("{}", serde_json::to_string_pretty(&body).unwrap());
            }
            return Ok(());
        }

        if matches.get_flag("json-body-schema") {
            let schema = schemars::schema_for!(types::UnoBody);
            print!("{}", progenitor_client::render_body_schema(&schema));
            return Ok(());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value)
                .with_context(|| format!("failed to read {}", value.display()))?;
            let body_value = serde_json::from_str::<types::UnoBody>(&body_txt)
                .with_context(|| format!("failed to parse {}", value.display()))?;
            request = request.body(body_value);
        }

        self.config.execute_uno(matches, &mut request)?;
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
    /// Produce the JSON body for `--json-body-template`. There is no
    /// generic default: override this to launch an interactive body
    /// builder for your CLI. Returning `None` prints nothing (e.g.
    /// the user cancelled).
    fn build_body_template(
        &self,
        _schema: &schemars::schema::RootSchema,
    ) -> anyhow::Result<Option<serde_json::Value>> {
        anyhow::bail!(
            "--json-body-template requires an interactive body builder; override \
             CliConfig::build_body_template to provide one"
        )
    }

    fn execute_uno(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::Uno,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    Uno,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::Uno].into_iter()
    }

    pub fn operation_id(&self) -> &'static str {
        match self {
            CliCommand::Uno => "uno",
        }
    }
}
