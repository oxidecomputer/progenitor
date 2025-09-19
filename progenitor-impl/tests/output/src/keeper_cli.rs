use crate::keeper_builder::*;
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
            CliCommand::Enrol => Self::cli_enrol(),
            CliCommand::GlobalJobs => Self::cli_global_jobs(),
            CliCommand::Ping => Self::cli_ping(),
            CliCommand::ReportFinish => Self::cli_report_finish(),
            CliCommand::ReportOutput => Self::cli_report_output(),
            CliCommand::ReportStart => Self::cli_report_start(),
        }
    }

    pub fn cli_enrol() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("authorization")
                    .long("authorization")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("Authorization header (bearer token)"),
            )
            .arg(
                ::clap::Arg::new("host")
                    .long("host")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("key")
                    .long("key")
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

    pub fn cli_global_jobs() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("authorization")
                .long("authorization")
                .value_parser(::clap::value_parser!(::std::string::String))
                .required(true)
                .help("Authorization header (bearer token)"),
        )
    }

    pub fn cli_ping() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("authorization")
                .long("authorization")
                .value_parser(::clap::value_parser!(::std::string::String))
                .required(true)
                .help("Authorization header (bearer token)"),
        )
    }

    pub fn cli_report_finish() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("authorization")
                    .long("authorization")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("Authorization header (bearer token)"),
            )
            .arg(
                ::clap::Arg::new("duration-millis")
                    .long("duration-millis")
                    .value_parser(::clap::value_parser!(i32))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("end-time")
                    .long("end-time")
                    .value_parser(::clap::value_parser!(
                        ::chrono::DateTime<::chrono::offset::Utc>
                    ))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("exit-status")
                    .long("exit-status")
                    .value_parser(::clap::value_parser!(i32))
                    .required_unless_present("json-body"),
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
    }

    pub fn cli_report_output() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("authorization")
                    .long("authorization")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("Authorization header (bearer token)"),
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
    }

    pub fn cli_report_start() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("authorization")
                    .long("authorization")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(true)
                    .help("Authorization header (bearer token)"),
            )
            .arg(
                ::clap::Arg::new("script")
                    .long("script")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("start-time")
                    .long("start-time")
                    .value_parser(::clap::value_parser!(
                        ::chrono::DateTime<::chrono::offset::Utc>
                    ))
                    .required_unless_present("json-body"),
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
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::Enrol => self.execute_enrol(matches).await,
            CliCommand::GlobalJobs => self.execute_global_jobs(matches).await,
            CliCommand::Ping => self.execute_ping(matches).await,
            CliCommand::ReportFinish => self.execute_report_finish(matches).await,
            CliCommand::ReportOutput => self.execute_report_output(matches).await,
            CliCommand::ReportStart => self.execute_report_start(matches).await,
        }
    }

    pub async fn execute_enrol(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.enrol();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("host") {
            request = request.body_map(|body| body.host(value.clone()))
        }

        if let Some(value) = matches.get_one::<::std::string::String>("key") {
            request = request.body_map(|body| body.key(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::EnrolBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_enrol(matches, &mut request)?;
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

    pub async fn execute_global_jobs(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.global_jobs();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        self.config.execute_global_jobs(matches, &mut request)?;
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

    pub async fn execute_ping(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.ping();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        self.config.execute_ping(matches, &mut request)?;
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

    pub async fn execute_report_finish(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.report_finish();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        if let Some(value) = matches.get_one::<i32>("duration-millis") {
            request = request.body_map(|body| body.duration_millis(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<::chrono::DateTime<::chrono::offset::Utc>>("end-time")
        {
            request = request.body_map(|body| body.end_time(value.clone()))
        }

        if let Some(value) = matches.get_one::<i32>("exit-status") {
            request = request.body_map(|body| body.exit_status(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ReportFinishBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_report_finish(matches, &mut request)?;
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

    pub async fn execute_report_output(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.report_output();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ReportOutputBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_report_output(matches, &mut request)?;
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

    pub async fn execute_report_start(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.report_start();
        if let Some(value) = matches.get_one::<::std::string::String>("authorization") {
            request = request.authorization(value.clone());
        }

        if let Some(value) = matches.get_one::<::std::string::String>("script") {
            request = request.body_map(|body| body.script(value.clone()))
        }

        if let Some(value) =
            matches.get_one::<::chrono::DateTime<::chrono::offset::Utc>>("start-time")
        {
            request = request.body_map(|body| body.start_time(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::ReportStartBody>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_report_start(matches, &mut request)?;
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
    fn execute_enrol(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::Enrol,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_global_jobs(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::GlobalJobs,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_ping(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::Ping,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_report_finish(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ReportFinish,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_report_output(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ReportOutput,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_report_start(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ReportStart,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    Enrol,
    GlobalJobs,
    Ping,
    ReportFinish,
    ReportOutput,
    ReportStart,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::Enrol,
            CliCommand::GlobalJobs,
            CliCommand::Ping,
            CliCommand::ReportFinish,
            CliCommand::ReportOutput,
            CliCommand::ReportStart,
        ]
        .into_iter()
    }
}
