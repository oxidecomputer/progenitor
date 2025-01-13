use crate::propolis_server_builder::*;
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
            CliCommand::InstanceGet => Self::cli_instance_get(),
            CliCommand::InstanceEnsure => Self::cli_instance_ensure(),
            CliCommand::InstanceIssueCrucibleSnapshotRequest => {
                Self::cli_instance_issue_crucible_snapshot_request()
            }
            CliCommand::InstanceMigrateStatus => Self::cli_instance_migrate_status(),
            CliCommand::InstanceSerial => Self::cli_instance_serial(),
            CliCommand::InstanceStatePut => Self::cli_instance_state_put(),
            CliCommand::InstanceStateMonitor => Self::cli_instance_state_monitor(),
        }
    }

    pub fn cli_instance_get() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_instance_ensure() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("cloud-init-bytes")
                    .long("cloud-init-bytes")
                    .value_parser(::clap::value_parser!(::std::string::String))
                    .required(false),
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

    pub fn cli_instance_issue_crucible_snapshot_request() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("id")
                    .long("id")
                    .value_parser(::clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("snapshot-id")
                    .long("snapshot-id")
                    .value_parser(::clap::value_parser!(uuid::Uuid))
                    .required(true),
            )
            .about("Issue a snapshot request to a crucible backend")
    }

    pub fn cli_instance_migrate_status() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("migration-id")
                    .long("migration-id")
                    .value_parser(::clap::value_parser!(uuid::Uuid))
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

    pub fn cli_instance_serial() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_instance_state_put() -> ::clap::Command {
        ::clap::Command::new("")
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

    pub fn cli_instance_state_monitor() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("gen")
                    .long("gen")
                    .value_parser(::clap::value_parser!(u64))
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

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::InstanceGet => self.execute_instance_get(matches).await,
            CliCommand::InstanceEnsure => self.execute_instance_ensure(matches).await,
            CliCommand::InstanceIssueCrucibleSnapshotRequest => {
                self.execute_instance_issue_crucible_snapshot_request(matches)
                    .await
            }
            CliCommand::InstanceMigrateStatus => {
                self.execute_instance_migrate_status(matches).await
            }
            CliCommand::InstanceSerial => self.execute_instance_serial(matches).await,
            CliCommand::InstanceStatePut => self.execute_instance_state_put(matches).await,
            CliCommand::InstanceStateMonitor => self.execute_instance_state_monitor(matches).await,
        }
    }

    pub async fn execute_instance_get(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.instance_get();
        self.config.execute_instance_get(matches, &mut request)?;
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

    pub async fn execute_instance_ensure(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_ensure();
        if let Some(value) = matches.get_one::<::std::string::String>("cloud-init-bytes") {
            request = request.body_map(|body| body.cloud_init_bytes(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceEnsureRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_instance_ensure(matches, &mut request)?;
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

    pub async fn execute_instance_issue_crucible_snapshot_request(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_issue_crucible_snapshot_request();
        if let Some(value) = matches.get_one::<uuid::Uuid>("id") {
            request = request.id(value.clone());
        }

        if let Some(value) = matches.get_one::<uuid::Uuid>("snapshot-id") {
            request = request.snapshot_id(value.clone());
        }

        self.config
            .execute_instance_issue_crucible_snapshot_request(matches, &mut request)?;
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

    pub async fn execute_instance_migrate_status(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_migrate_status();
        if let Some(value) = matches.get_one::<uuid::Uuid>("migration-id") {
            request = request.body_map(|body| body.migration_id(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceMigrateStatusRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_migrate_status(matches, &mut request)?;
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

    pub async fn execute_instance_serial(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_serial();
        self.config.execute_instance_serial(matches, &mut request)?;
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

    pub async fn execute_instance_state_put(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_state_put();
        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceStateRequested>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_state_put(matches, &mut request)?;
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

    pub async fn execute_instance_state_monitor(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.instance_state_monitor();
        if let Some(value) = matches.get_one::<u64>("gen") {
            request = request.body_map(|body| body.gen(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value =
                serde_json::from_str::<types::InstanceStateMonitorRequest>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_instance_state_monitor(matches, &mut request)?;
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
    fn execute_instance_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_ensure(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceEnsure,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_issue_crucible_snapshot_request(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceIssueCrucibleSnapshotRequest,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_migrate_status(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceMigrateStatus,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_serial(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceSerial,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_state_put(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceStatePut,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_instance_state_monitor(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::InstanceStateMonitor,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    InstanceGet,
    InstanceEnsure,
    InstanceIssueCrucibleSnapshotRequest,
    InstanceMigrateStatus,
    InstanceSerial,
    InstanceStatePut,
    InstanceStateMonitor,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::InstanceGet,
            CliCommand::InstanceEnsure,
            CliCommand::InstanceIssueCrucibleSnapshotRequest,
            CliCommand::InstanceMigrateStatus,
            CliCommand::InstanceSerial,
            CliCommand::InstanceStatePut,
            CliCommand::InstanceStateMonitor,
        ]
        .into_iter()
    }
}
