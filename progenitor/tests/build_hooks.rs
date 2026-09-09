// Copyright 2026 Oxide Computer Company

//! Validate the signatures with which pre and post hooks are invoked.

fn add_header(req: &mut reqwest::Request) {
    req.headers_mut().insert(
        "x-pre-hook",
        reqwest::header::HeaderValue::from_static("present"),
    );
}

fn observe(_req: &reqwest::Request) {}

fn observe_result(_result: &Result<reqwest::Response, reqwest::Error>) {}

// The sync pre_hook receives `&mut reqwest::Request` so that it can modify
// the request before it's sent.
mod pre_hook_mut {
    progenitor::generate_api!(
        spec = "../sample_openapi/keeper.json",
        pre_hook = crate::add_header,
        post_hook = crate::observe_result,
    );
}

// A pre_hook written against `&reqwest::Request` must continue to compile:
// the generated call site passes `&mut request`, which coerces to `&_`.
mod pre_hook_ref {
    progenitor::generate_api!(
        spec = "../sample_openapi/keeper.json",
        pre_hook = crate::observe,
    );
}
