# Progenitor

Progenitor is a Rust crate for generating opinionated clients from API
descriptions in the OpenAPI 3.0.x specification. It makes use of Rust
futures for `async` API calls and `Streams` for paginated interfaces.

It generates a type called `Client` with methods that correspond to the
operations specified in the OpenAPI document.

Progenitor can also generate a CLI to interact with an OpenAPI service
instance, and [`httpmock`](https://crates.io/crates/httpmock) helpers to
create a strongly typed mock of the OpenAPI service.

The primary target is OpenAPI documents emitted by
[Dropshot](https://github.com/oxidecomputer/dropshot)-generated APIs, but it
can be used for many OpenAPI documents. As OpenAPI covers a wide range of APIs,
Progenitor may fail for some OpenAPI documents. If you encounter a problem, you
can help the project by filing an issue that includes the OpenAPI document that
produced the problem.

## Using Progenitor

There are three different ways of using the `progenitor` crate. The one you
choose will depend on your use case and preferences.

### Macro

The simplest way to use Progenitor is via its `generate_api!` macro.

In a source file (often `main.rs`, `lib.rs`, or `mod.rs`) simply invoke the
macro:

```rust
generate_api!("path/to/openapi_document.json");
```

You'll need to add the following to `Cargo.toml`:

```toml
[dependencies]
futures = "0.3"
progenitor = { git = "https://github.com/oxidecomputer/progenitor" }
reqwest = { version = "0.12", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
```

In addition, if the OpenAPI document contains string types with the `format`
field set to `date` or `date-time`, include

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

Similarly, if there is a `format` field set to `uuid`:

```toml
[dependencies]
uuid = { version = "1.0.0", features = ["serde", "v4"] }
```

And if there are any websocket channel endpoints:

```toml
[dependencies]
base64 = "0.21"
rand = "0.8"
```

If types include regular expression validation:

```toml
[dependencies]
regress = "0.4.1"
```

The macro has some additional fancy options to control the generated code:

```rust
generate_api!(
    spec = "path/to/openapi_document.json",      // The OpenAPI document
    interface = Builder,                         // Choose positional (default) or builder style
    tags = Separate,                             // Tags may be Merged or Separate (default)
    inner_type = my_client::InnerType,           // Client inner type available to pre and post hooks
    pre_hook = closure::or::path::to::function,  // Hook invoked before issuing the HTTP request
    post_hook = closure::or::path::to::function, // Hook invoked prior to receiving the HTTP response
    derives = [ schemars::JsonSchema ],          // Additional derive macros applied to generated types
);
```

Note that the macro will be re-evaluated when the `spec` OpenAPI document
changes (when its mtime is updated).

### `build.rs`

Progenitor includes an interface appropriate for use in a
[`build.rs`](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
file. While slightly more onerous than the macro, a builder has the advantage of making the generated code visible.
The capability of generating a CLI and `httpmock` helpers is only available using `build.rs`
and the `Generator` functions `cli` and `httpmock` respectively.

The `build.rs` file should look something like this:

```rust
fn main() {
    let src = "../sample_openapi/keeper.json";
    println!("cargo:rerun-if-changed={}", src);
    let file = std::fs::File::open(src).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = progenitor::Generator::default();

    let tokens = generator.generate_tokens(&spec).unwrap();
    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
    out_file.push("codegen.rs");

    std::fs::write(out_file, content).unwrap();
}
```

In a source file (often `main.rs`, `lib.rs`, or `mod.rs`) include the generated
code:

```rust
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
```

You'll need to add the following to `Cargo.toml`:

```toml
[dependencies]
futures = "0.3"
progenitor-client = { git = "https://github.com/oxidecomputer/progenitor" }
reqwest = { version = "0.12", features = ["json", "stream"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
prettyplease = "0.2.22"
progenitor = { git = "https://github.com/oxidecomputer/progenitor" }
serde_json = "1.0"
syn = "2.0"
```

(`chrono`, `uuid`, `base64`, and `rand` as above)

Note that `progenitor` is used by `build.rs`, but the generated code required
`progenitor-client`.

### Static Crate

Progenitor can be run to emit a stand-alone crate for the generated client.
This ensures no unexpected changes (e.g. from updates to progenitor). It is
however, the most manual way to use Progenitor.

Usage:

```
cargo progenitor

Options:
    -i INPUT            OpenAPI definition document (JSON or YAML)
    -o OUTPUT           Generated Rust crate directory
    -n CRATE            Target Rust crate name
    -v VERSION          Target Rust crate version
```

For example:

```
cargo install cargo-progenitor
cargo progenitor -i sample_openapi/keeper.json -o keeper -n keeper -v 0.1.0
```

... or within the repo:
```
cargo run --bin cargo-progenitor -- progenitor -i sample_openapi/keeper.json -o keeper -n keeper -v 0.1.0
```

This will produce a package in the specified directory.

Options `--license` and `--registry-name` may also be used to improve metadata
before publishing the static crate.

The output will use the published `progenitor-client` crate by default
if progenitor was built from a released version.  However, when using progenitor
built from the repository, the `progenitor-client` will be inlined into the
static crate by default.  The command line flag `--include-client` can be used
to override the default behaviour.

To ensure the output has no persistent dependency on Progenitor, enable `--include-client`.

Here is an excerpt from the emitted `Cargo.toml`:

```toml
[dependencies]
bytes = "1.3.0"
chrono = { version = "0.4.23", default-features=false, features = ["serde"] }
futures-core = "0.3.25"
percent-encoding = "2.2.0"
reqwest = { version = "0.12.4", default-features=false, features = ["json", "stream"] }
serde = { version = "1.0.152", features = ["derive"] }
serde_urlencoded = "0.7.1"
```

The dependency versions in the generated `Cargo.toml` are the same as the
versions that were used when progenitor was built.

Note that there is a dependency on `percent-encoding` which macro- and
build.rs-generated clients is included from `progenitor-client`.

## Generation Styles

Progenitor can generate two distinct interface styles: positional and builder
(described below). The choice is simply a matter of preference that many vary
by API and taste.

## Positional (current default)

The "positional" style generates `Client` methods that accept parameters in
order, for example:

```rust
impl Client {
    pub async fn instance_create<'a>(
        &'a self,
        organization_name: &'a types::Name,
        project_name: &'a types::Name,
        body: &'a types::InstanceCreate,
    ) -> Result<ResponseValue<types::Instance>, Error<types::Error>> {
        // ...
    }
}
```

A caller invokes this interface by specifying parameters by position:

```rust
let result = client.instance_create(org, proj, body).await?;
```

Note that the type of each parameter must match precisely--no conversion is
done implicitly.

## Builder

The "builder" style generates `Client` methods that produce a builder struct.
API parameters are applied to that builder, and then the builder is executed
(via a `send` method). The code is more extensive and more complex to enable
simpler and more legible consumers:

```rust
impl Client
    pub fn instance_create(&self) -> builder::InstanceCreate {
        builder::InstanceCreate::new(self)
    }
}

mod builder {
    pub struct InstanceCreate<'a> {
        client: &'a super::Client,
        organization_name: Result<types::Name, String>,
        project_name: Result<types::Name, String>,
        body: Result<types::InstanceCreate, String>,
    }

    impl<'a> InstanceCreate<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            // ...
        }

        pub fn organization_name<V>(mut self, value: V) -> Self
        where
            V: TryInto<types::Name>,
        {
            // ...
        }

        pub fn project_name<V>(mut self, value: V) -> Self
        where
            V: TryInto<types::Name>,
        {
            // ...
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: TryInto<types::InstanceCreate>,
        {
            // ...
        }

        pub async fn send(self) ->
            Result<ResponseValue<types::Instance>, Error<types::Error>>
        {
            // ...
        }
    }
}
```

Note that, unlike positional generation, consumers can supply compatible
(rather than invariant) parameters:

```rust
let result = client
    .instance_create()
    .organization_name("org")
    .project_name("proj")
    .body(body)
    .send()
    .await?;
```

The string parameters will implicitly have `TryFrom::try_from()` invoked on
them. Failed conversions or missing required parameters will result in an
`Error` result from the `send()` call.

Generated `struct` types also have builders so that the `body` parameter can be
constructed inline:

```rust
let result = client
    .instance_create()
    .organization_name("org")
    .project_name("proj")
    .body(types::InstanceCreate::builder()
        .name("...")
        .description("...")
        .hostname("...")
        .ncpus(types::InstanceCpuCount(4))
        .memory(types::ByteCount(1024 * 1024 * 1024)),
    )
    .send()
    .await?;
```

Consumers do not need to specify parameters and struct properties that are not
required or for which the API specifies defaults. Neat!
