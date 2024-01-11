use crate::param_collision_builder::*;
pub struct Cli<T: CliOverride = ()> {
    client: Client,
    over: T,
}

impl Cli {
    pub fn new(client: Client) -> Self {
        Self { client, over: () }
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
    }
}

impl<T: CliOverride> Cli<T> {
    pub fn new_with_override(client: Client, over: T) -> Self {
        Self { client, over }
    }

    pub async fn execute(&self, cmd: CliCommand, matches: &clap::ArgMatches) {
        match cmd {
            CliCommand::KeyGet => {
                self.execute_key_get(matches).await;
            }
        }
    }

    pub async fn execute_key_get(&self, matches: &clap::ArgMatches) {
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

        self.over.execute_key_get(matches, &mut request).unwrap();
        let result = request.send().await;
        match result {
            Ok(r) => {
                println!("success\n{:#?}", r)
            }
            Err(r) => {
                println!("success\n{:#?}", r)
            }
        }
    }
}

pub trait CliOverride {
    fn execute_key_get(
        &self,
        matches: &clap::ArgMatches,
        request: &mut builder::KeyGet,
    ) -> Result<(), String> {
        Ok(())
    }
}

impl CliOverride for () {}
#[derive(Copy, Clone, Debug)]
pub enum CliCommand {
    KeyGet,
}

impl CliCommand {
    pub fn iter() -> impl Iterator<Item = CliCommand> {
        vec![CliCommand::KeyGet].into_iter()
    }
}
