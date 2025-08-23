// Copyright 2022 Oxide Computer Company

use test_progenitor_compilation::cli_tokens;

#[test]
fn test_keeper_compilation() {
    cli_tokens!("keeper.json");
}

#[test]
fn test_buildomat_compilation() {
    cli_tokens!("buildomat.json");
}

#[test]
fn test_nexus_compilation() {
    cli_tokens!("nexus.json");
}

#[test]
fn test_propolis_server_compilation() {
    cli_tokens!("propolis-server.json");
}

#[test]
fn test_param_override_compilation() {
    cli_tokens!("param-overrides.json");
}

#[test]
fn test_yaml_compilation() {
    cli_tokens!("param-overrides.yaml");
}

#[test]
fn test_param_collision_compilation() {
    cli_tokens!("param-collision.json");
}

// TODO this file is full of inconsistencies and incorrectly specified types.
// It's an interesting test to consider whether we try to do our best to
// interpret the intent or just fail.
// #[test]
// fn test_github() {
//     cli_tokens!("api.github.com.json");
// }
