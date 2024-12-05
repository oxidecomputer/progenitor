use crate::buildomat_builder::*;
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
            CliCommand::ControlHold => Self::cli_control_hold(),
            CliCommand::ControlResume => Self::cli_control_resume(),
            CliCommand::TaskGet => Self::cli_task_get(),
            CliCommand::TasksGet => Self::cli_tasks_get(),
            CliCommand::TaskSubmit => Self::cli_task_submit(),
            CliCommand::TaskEventsGet => Self::cli_task_events_get(),
            CliCommand::TaskOutputsGet => Self::cli_task_outputs_get(),
            CliCommand::TaskOutputDownload => Self::cli_task_output_download(),
            CliCommand::UserCreate => Self::cli_user_create(),
            CliCommand::Whoami => Self::cli_whoami(),
            CliCommand::WhoamiPutName => Self::cli_whoami_put_name(),
            CliCommand::WorkerBootstrap => Self::cli_worker_bootstrap(),
            CliCommand::WorkerPing => Self::cli_worker_ping(),
            CliCommand::WorkerTaskAppend => Self::cli_worker_task_append(),
            CliCommand::WorkerTaskUploadChunk => Self::cli_worker_task_upload_chunk(),
            CliCommand::WorkerTaskComplete => Self::cli_worker_task_complete(),
            CliCommand::WorkerTaskAddOutput => Self::cli_worker_task_add_output(),
            CliCommand::WorkersList => Self::cli_workers_list(),
            CliCommand::WorkersRecycle => Self::cli_workers_recycle(),
        }
    }

    pub fn cli_control_hold() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_control_resume() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_task_get() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("task")
                .long("task")
                .value_parser(::clap::value_parser!(String))
                .required(true),
        )
    }

    pub fn cli_tasks_get() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_task_submit() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("script")
                    .long("script")
                    .value_parser(::clap::value_parser!(String))
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

    pub fn cli_task_events_get() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("minseq")
                    .long("minseq")
                    .value_parser(::clap::value_parser!(u32))
                    .required(false),
            )
            .arg(
                ::clap::Arg::new("task")
                    .long("task")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
            )
    }

    pub fn cli_task_outputs_get() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("task")
                .long("task")
                .value_parser(::clap::value_parser!(String))
                .required(true),
        )
    }

    pub fn cli_task_output_download() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("output")
                    .long("output")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("task")
                    .long("task")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
            )
    }

    pub fn cli_user_create() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("name")
                    .long("name")
                    .value_parser(::clap::value_parser!(String))
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

    pub fn cli_whoami() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_whoami_put_name() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_worker_bootstrap() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("bootstrap")
                    .long("bootstrap")
                    .value_parser(::clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("token")
                    .long("token")
                    .value_parser(::clap::value_parser!(String))
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

    pub fn cli_worker_ping() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_worker_task_append() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("payload")
                    .long("payload")
                    .value_parser(::clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("stream")
                    .long("stream")
                    .value_parser(::clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("task")
                    .long("task")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
            )
            .arg(
                ::clap::Arg::new("time")
                    .long("time")
                    .value_parser(::clap::value_parser!(chrono::DateTime<chrono::offset::Utc>))
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

    pub fn cli_worker_task_upload_chunk() -> ::clap::Command {
        ::clap::Command::new("").arg(
            ::clap::Arg::new("task")
                .long("task")
                .value_parser(::clap::value_parser!(String))
                .required(true),
        )
    }

    pub fn cli_worker_task_complete() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("failed")
                    .long("failed")
                    .value_parser(::clap::value_parser!(bool))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("task")
                    .long("task")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
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

    pub fn cli_worker_task_add_output() -> ::clap::Command {
        ::clap::Command::new("")
            .arg(
                ::clap::Arg::new("path")
                    .long("path")
                    .value_parser(::clap::value_parser!(String))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("size")
                    .long("size")
                    .value_parser(::clap::value_parser!(i64))
                    .required_unless_present("json-body"),
            )
            .arg(
                ::clap::Arg::new("task")
                    .long("task")
                    .value_parser(::clap::value_parser!(String))
                    .required(true),
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

    pub fn cli_workers_list() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub fn cli_workers_recycle() -> ::clap::Command {
        ::clap::Command::new("")
    }

    pub async fn execute(
        &self,
        cmd: CliCommand,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        match cmd {
            CliCommand::ControlHold => self.execute_control_hold(matches).await,
            CliCommand::ControlResume => self.execute_control_resume(matches).await,
            CliCommand::TaskGet => self.execute_task_get(matches).await,
            CliCommand::TasksGet => self.execute_tasks_get(matches).await,
            CliCommand::TaskSubmit => self.execute_task_submit(matches).await,
            CliCommand::TaskEventsGet => self.execute_task_events_get(matches).await,
            CliCommand::TaskOutputsGet => self.execute_task_outputs_get(matches).await,
            CliCommand::TaskOutputDownload => self.execute_task_output_download(matches).await,
            CliCommand::UserCreate => self.execute_user_create(matches).await,
            CliCommand::Whoami => self.execute_whoami(matches).await,
            CliCommand::WhoamiPutName => self.execute_whoami_put_name(matches).await,
            CliCommand::WorkerBootstrap => self.execute_worker_bootstrap(matches).await,
            CliCommand::WorkerPing => self.execute_worker_ping(matches).await,
            CliCommand::WorkerTaskAppend => self.execute_worker_task_append(matches).await,
            CliCommand::WorkerTaskUploadChunk => {
                self.execute_worker_task_upload_chunk(matches).await
            }
            CliCommand::WorkerTaskComplete => self.execute_worker_task_complete(matches).await,
            CliCommand::WorkerTaskAddOutput => self.execute_worker_task_add_output(matches).await,
            CliCommand::WorkersList => self.execute_workers_list(matches).await,
            CliCommand::WorkersRecycle => self.execute_workers_recycle(matches).await,
        }
    }

    pub async fn execute_control_hold(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.control_hold();
        self.config.execute_control_hold(matches, &mut request)?;
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

    pub async fn execute_control_resume(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.control_resume();
        self.config.execute_control_resume(matches, &mut request)?;
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

    pub async fn execute_task_get(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.task_get();
        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        self.config.execute_task_get(matches, &mut request)?;
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

    pub async fn execute_tasks_get(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.tasks_get();
        self.config.execute_tasks_get(matches, &mut request)?;
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

    pub async fn execute_task_submit(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.task_submit();
        if let Some(value) = matches.get_one::<String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("script") {
            request = request.body_map(|body| body.script(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::TaskSubmit>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_task_submit(matches, &mut request)?;
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

    pub async fn execute_task_events_get(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.task_events_get();
        if let Some(value) = matches.get_one::<u32>("minseq") {
            request = request.minseq(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        self.config.execute_task_events_get(matches, &mut request)?;
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

    pub async fn execute_task_outputs_get(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.task_outputs_get();
        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        self.config
            .execute_task_outputs_get(matches, &mut request)?;
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

    pub async fn execute_task_output_download(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.task_output_download();
        if let Some(value) = matches.get_one::<String>("output") {
            request = request.output(value.clone());
        }

        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        self.config
            .execute_task_output_download(matches, &mut request)?;
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

    pub async fn execute_user_create(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.user_create();
        if let Some(value) = matches.get_one::<String>("name") {
            request = request.body_map(|body| body.name(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::UserCreate>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config.execute_user_create(matches, &mut request)?;
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

    pub async fn execute_whoami(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.whoami();
        self.config.execute_whoami(matches, &mut request)?;
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

    pub async fn execute_whoami_put_name(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.whoami_put_name();
        self.config.execute_whoami_put_name(matches, &mut request)?;
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

    pub async fn execute_worker_bootstrap(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.worker_bootstrap();
        if let Some(value) = matches.get_one::<String>("bootstrap") {
            request = request.body_map(|body| body.bootstrap(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("token") {
            request = request.body_map(|body| body.token(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::WorkerBootstrap>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_worker_bootstrap(matches, &mut request)?;
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

    pub async fn execute_worker_ping(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.worker_ping();
        self.config.execute_worker_ping(matches, &mut request)?;
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

    pub async fn execute_worker_task_append(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.worker_task_append();
        if let Some(value) = matches.get_one::<String>("payload") {
            request = request.body_map(|body| body.payload(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("stream") {
            request = request.body_map(|body| body.stream(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        if let Some(value) = matches.get_one::<chrono::DateTime<chrono::offset::Utc>>("time") {
            request = request.body_map(|body| body.time(value.clone()))
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::WorkerAppendTask>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_worker_task_append(matches, &mut request)?;
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

    pub async fn execute_worker_task_upload_chunk(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.worker_task_upload_chunk();
        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        self.config
            .execute_worker_task_upload_chunk(matches, &mut request)?;
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

    pub async fn execute_worker_task_complete(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.worker_task_complete();
        if let Some(value) = matches.get_one::<bool>("failed") {
            request = request.body_map(|body| body.failed(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::WorkerCompleteTask>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_worker_task_complete(matches, &mut request)?;
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

    pub async fn execute_worker_task_add_output(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.worker_task_add_output();
        if let Some(value) = matches.get_one::<String>("path") {
            request = request.body_map(|body| body.path(value.clone()))
        }

        if let Some(value) = matches.get_one::<i64>("size") {
            request = request.body_map(|body| body.size(value.clone()))
        }

        if let Some(value) = matches.get_one::<String>("task") {
            request = request.task(value.clone());
        }

        if let Some(value) = matches.get_one::<std::path::PathBuf>("json-body") {
            let body_txt = std::fs::read_to_string(value).unwrap();
            let body_value = serde_json::from_str::<types::WorkerAddOutput>(&body_txt).unwrap();
            request = request.body(body_value);
        }

        self.config
            .execute_worker_task_add_output(matches, &mut request)?;
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

    pub async fn execute_workers_list(&self, matches: &::clap::ArgMatches) -> anyhow::Result<()> {
        let mut request = self.client.workers_list();
        self.config.execute_workers_list(matches, &mut request)?;
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

    pub async fn execute_workers_recycle(
        &self,
        matches: &::clap::ArgMatches,
    ) -> anyhow::Result<()> {
        let mut request = self.client.workers_recycle();
        self.config.execute_workers_recycle(matches, &mut request)?;
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
    fn execute_control_hold(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ControlHold,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_control_resume(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::ControlResume,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_task_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TaskGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_tasks_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TasksGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_task_submit(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TaskSubmit,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_task_events_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TaskEventsGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_task_outputs_get(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TaskOutputsGet,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_task_output_download(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::TaskOutputDownload,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_user_create(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::UserCreate,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_whoami(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::Whoami,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_whoami_put_name(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WhoamiPutName,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_bootstrap(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerBootstrap,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_ping(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerPing,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_task_append(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerTaskAppend,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_task_upload_chunk(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerTaskUploadChunk,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_task_complete(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerTaskComplete,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_worker_task_add_output(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkerTaskAddOutput,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_workers_list(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkersList,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn execute_workers_recycle(
        &self,
        matches: &::clap::ArgMatches,
        request: &mut builder::WorkersRecycle,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    ControlHold,
    ControlResume,
    TaskGet,
    TasksGet,
    TaskSubmit,
    TaskEventsGet,
    TaskOutputsGet,
    TaskOutputDownload,
    UserCreate,
    Whoami,
    WhoamiPutName,
    WorkerBootstrap,
    WorkerPing,
    WorkerTaskAppend,
    WorkerTaskUploadChunk,
    WorkerTaskComplete,
    WorkerTaskAddOutput,
    WorkersList,
    WorkersRecycle,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![
            CliCommand::ControlHold,
            CliCommand::ControlResume,
            CliCommand::TaskGet,
            CliCommand::TasksGet,
            CliCommand::TaskSubmit,
            CliCommand::TaskEventsGet,
            CliCommand::TaskOutputsGet,
            CliCommand::TaskOutputDownload,
            CliCommand::UserCreate,
            CliCommand::Whoami,
            CliCommand::WhoamiPutName,
            CliCommand::WorkerBootstrap,
            CliCommand::WorkerPing,
            CliCommand::WorkerTaskAppend,
            CliCommand::WorkerTaskUploadChunk,
            CliCommand::WorkerTaskComplete,
            CliCommand::WorkerTaskAddOutput,
            CliCommand::WorkersList,
            CliCommand::WorkersRecycle,
        ]
        .into_iter()
    }
}
