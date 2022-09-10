# Implementing the client

Once you have generated your client, you'll notice there are two methods to
create a new `Client`; `Client::new()` and `Client::new_with_client()`.

The first method will create a basic client without any headers or
customizations that aren't already in your OpenAPI specification file:

```rust
let client = Client::new("https://foo/bar");
```

Should you need to set custom headers or other `reqwest::ClientBuilder`
methods, you can use `Client::new_with_client()` as shown below.

```rust
let mut val = reqwest::header::HeaderValue::from_static("super-secret");
val.set_sensitive(true);
let mut headers = reqwest::header::HeaderMap::new();
headers.insert(reqwest::header::AUTHORIZATION, val);
// Insert more headers if necessary

let client_builder = reqwest::ClientBuilder::new()
    // Set custom timeout
    .connect_timeout(Duration::new(60, 0))
    // Set custom headers
    .default_headers(headers)
    .build()
    .unwrap();

let client Client::new_with_client("https://foo/bar", client_builder);
```

For more information on available methods, see the
[reqwest](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
documentation.