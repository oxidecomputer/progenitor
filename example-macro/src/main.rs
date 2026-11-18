// Copyright 2026 Oxide Computer Company

use progenitor::generate_api;

generate_api!(
    spec = "../sample_openapi/keeper.json",
    derives = [schemars::JsonSchema],
);

// Modify the client behavior by executing code before and after each server
// request.
impl ClientHooks for Client {
    async fn pre<E>(
        &self,
        request: &mut reqwest::Request,
        _info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        println!("doing this {:?}", request);
        add_auth_headers(request).await?;
        Ok(())
    }

    async fn post<E>(
        &self,
        result: &reqwest::Result<reqwest::Response>,
        _info: &OperationInfo,
    ) -> std::result::Result<(), Error<E>> {
        all_done(result);
        Ok(())
    }
}

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
