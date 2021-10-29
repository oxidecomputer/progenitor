// Copyright 2021 Oxide Computer Company

use progenitor::generate_api;

generate_api!(
    "../sample_openapi/keeper.json",
    (),
    |_, request| {
        println!("doing this {:?}", request);
    },
    crate::all_done
);

fn all_done(_: &(), _result: &reqwest::Result<reqwest::Response>) {}

fn main() {}
