use crate::query_param_defaults_builder::*;
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
            CliCommand::UploadFile => Self::cli_upload_file(),
        }
    }

    pub fn cli_upload_file() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("required-param")
                    .long("required-param")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("supports-all-drives")
                    .long("supports-all-drives")
                    .value_parser(::clap::value_parser!(bool))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("upload-type")
                    .long("upload-type")
                    .value_parser(::clap::builder::TypedValueParser::map(
                        ::clap::builder::PossibleValuesParser::new([
                            types::UploadFileUploadType::Multipart.to_string(),
                        ]),
                        |s| types::UploadFileUploadType::try_from(s).unwrap(),
                    ))
                    .required(false),
            )
            .about("Upload a file")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::UploadFile => self.execute_upload_file(matches).await,
        }
    }

    pub async fn execute_upload_file(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.upload_file();
        if let Some(value) = matches.get_one::<::std::string::String>("required-param") {
            request = request.required_param(value.clone());
        }

        if let Some(value) = matches.get_one::<bool>("supports-all-drives") {
            request = request.supports_all_drives(value.clone());
        }

        if let Some(value) = matches.get_one::<types::UploadFileUploadType>("upload-type") {
            request = request.upload_type(value.clone());
        }

        self.config.execute_upload_file(matches, &mut request)?;
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
    fn execute_upload_file(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UploadFile,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    UploadFile,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::UploadFile].into_iter()
    }
}
