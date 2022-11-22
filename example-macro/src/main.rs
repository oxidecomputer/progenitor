// Copyright 2022 Oxide Computer Company

use progenitor::generate_api;

generate_api!(
    spec = "../sample_openapi/keeper.json",
    inner_type = (),
    pre_hook = (|_, request| {
        println!("doing this {:?}", request);
    }),
    post_hook = crate::all_done,
    derives = [schemars::JsonSchema],
);

fn all_done(_: &(), _result: &reqwest_middleware::Result<reqwest::Response>) {}

mod buildomat {
    use progenitor::generate_api;

    generate_api!("../sample_openapi/buildomat.json");
}

fn main() {}
