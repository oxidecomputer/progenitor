# Implementing the client

A generated SDK contains a `Client` type with two creation methods,
`Client::new()` and `Client::new_with_client()`.

The first method will create a basic client without any headers or
customizations that aren't specified in your OpenAPI specification file:

```rust
let client = Client::new("https://foo/bar").expect("failed to create client");
```

Should you need to set custom headers or other `reqwest::ClientBuilder`
methods, you can use `Client::new_with_client()`:

```rust
let mut val = reqwest::header::HeaderValue::from_static("super-secret");
val.set_sensitive(true);
let mut headers = reqwest::header::HeaderMap::new();
headers.insert(reqwest::header::AUTHORIZATION, val);
// Insert more headers if necessary

let reqwest_client = reqwest::ClientBuilder::new()
    // Set custom timeout
    .connect_timeout(Duration::new(60, 0))
    // Set custom headers
    .default_headers(headers)
    .build()
    .expect("failed to build the reqwest client");

let client = Client::new_with_client("https://foo/bar", reqwest_client);
```

For more information on available methods, see the
[reqwest](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
documentation.
