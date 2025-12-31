# Implementing the client

Once you have generated your client, all generated clients have a `Client::new()` method:

```rust
let client = Client::new("https://foo/bar");
```

## reqwest backend

For the reqwest backend, there is also a `Client::new_with_client()` method that allows you to customize the underlying HTTP client.

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

let client = Client::new_with_client("https://foo/bar", client_builder);
```

For more information on available methods, see the
[reqwest](https://docs.rs/reqwest/latest/reqwest/struct.ClientBuilder.html)
documentation.

## gloo backend

The gloo backend does not have `Client::new_with_client()` because it uses the browser's native Fetch API. HTTP behavior (timeouts, headers, credentials, etc.) is controlled by the browser and request options.

For custom request behavior, you can implement the `ClientHooks` trait to intercept and modify requests:

```rust
impl ClientHooks for &Client {
    async fn pre<E>(
        &self,
        request: &mut gloo_net::http::Request,
        info: &OperationInfo,
    ) -> Result<(), Error<E>> {
        // Modify request before sending
        Ok(())
    }
}
```
