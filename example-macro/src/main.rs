// Copyright 2022 Oxide Computer Company

use progenitor::generate_api;

generate_api!(
    spec = "../sample_openapi/keeper.json",
    pre_hook = (|request| {
        println!("doing this {request:?}");
    }),
    pre_hook_async = crate::add_auth_headers,
    post_hook = crate::all_done,
    derives = [schemars::JsonSchema],
);

async fn add_auth_headers(
    req: &mut reqwest::Request,
) -> Result<(), reqwest::header::InvalidHeaderValue> {
    // You can perform asynchronous, fallible work in a request hook, then
    // modify the request right before it is transmitted to the server; e.g.,
    // for generating an authenticaiton signature based on the complete set of
    // request header values:
    req.headers_mut().insert(
        reqwest::header::AUTHORIZATION,
        reqwest::header::HeaderValue::from_str("legitimate")?,
    );

    Ok(())
}

fn all_done(_result: &reqwest::Result<reqwest::Response>) {}

mod buildomat {
    use progenitor::generate_api;

    generate_api!("../sample_openapi/buildomat.json");
}

fn main() {}
