use crate::multipart_related_test_builder::*;
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
            CliCommand::UploadSimple => Self::cli_upload_simple(),
            CliCommand::UploadMultipleFiles => Self::cli_upload_multiple_files(),
        }
    }

    pub fn cli_upload_file() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("file-content-type")
                    .long("file-content-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("MIME type for the file field"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Upload a file with metadata using multipart/related")
            .long_about("Uploads a file along with JSON metadata in a multipart/related request")
    }

    pub fn cli_upload_simple() -> ::clap::Command {
        ::clap::Command::new("").about("Simple upload using multipart/related")
    }

    pub fn cli_upload_multiple_files() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("attachment-content-type")
                    .long("attachment-content-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false)
                    .help("MIME type for the attachment field"),
            )
            .arg(
                ::clap::Arg::new("document-content-type")
                    .long("document-content-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("MIME type for the document field"),
            )
            .arg(
                ::clap::Arg::new("thumbnail-content-type")
                    .long("thumbnail-content-type")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body")
                    .help("MIME type for the thumbnail field"),
            )
            .arg(
                ::clap::Arg::new("json-body")
                    .long("json-body")
                    .value_name("JSON-FILE")
                    .required(true)
                    .value_parser(::clap::value_parser!(std::path::PathBuf))
                    .help("Path to a file that contains the full json body."),
            )
            .arg(
                ::clap::Arg::new("json-body-template")
                    .long("json-body-template")
                    .action(::clap::ArgAction::SetTrue)
                    .help("XXX"),
            )
            .about("Upload multiple files with metadata using multipart/related")
            .long_about(
                "Uploads multiple files along with JSON metadata in a single multipart/related \
                 request",
            )
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::UploadFile => self.execute_upload_file(matches).await,
            CliCommand::UploadSimple => self.execute_upload_simple(matches).await,
            CliCommand::UploadMultipleFiles => self.execute_upload_multiple_files(matches).await,
        }
    }

    pub async fn execute_upload_file(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.upload_file();
        if let Some(value) = matches.get_one::<::std::string::String>("file-content-type") {
            request = request.body_map(|body| body.file_content_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::UploadFileMultipartParts>(&body_txt).unwrap();
            request = request.body(body_value);
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

    pub async fn execute_upload_simple(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.upload_simple();
        self.config.execute_upload_simple(matches, &mut request)?;
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

    pub async fn execute_upload_multiple_files(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.upload_multiple_files();
        if let Some(value) = matches.get_one::<::std::string::String>("attachment-content-type") {
            request = request.body_map(|body| body.attachment_content_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("document-content-type") {
            request = request.body_map(|body| body.document_content_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("thumbnail-content-type") {
            request = request.body_map(|body| body.thumbnail_content_type(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::UploadMultipleFilesMultipartParts>(&body_txt)
                    .unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_upload_multiple_files(matches, &mut request)?;
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

    fn execute_upload_simple(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UploadSimple,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_upload_multiple_files(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UploadMultipleFiles,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    UploadFile,
    UploadSimple,
    UploadMultipleFiles,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::UploadFile,
            CliCommand::UploadSimple,
            CliCommand::UploadMultipleFiles,
        ]
        .into_iter()
    }
}
