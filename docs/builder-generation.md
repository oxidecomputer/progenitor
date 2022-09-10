# Builder Generation

When the "builder" style is specified, Progenitor generates methods and builder
struct according to operations within an OpenAPI document. They take the
following general form:

```rust
impl Client {
    pub fn operation_name(&self) -> builder::InstanceCreate {
        builder::OperationName::new(self)
    }
}

mod builder {
    #[derive(Debug, Clone)]
    pub struct OperationName<'a> {
        client: &'a super::Client,

        path_parameter_1: Result<String, String>,
        path_parameter_2: Result<u32, String>,
        query_parameter_1: Result<String, String>,
        query_parameter_2: Result<Option<u32>, String>,
        body: Result<types::ThisOperationBody, String>,
    }

    impl<'a> OperationName<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                path_parameter_1: Err("path_parameter_1 was not initialized".to_string()),
                path_parameter_2: Err("path_parameter_2 was not initialized".to_string()),
                query_parameter_1: Err("query_parameter_1 was not initialized".to_string()),
                query_parameter_2: Ok(None),
                body: Err("body was not initialized".to_string()),
            }
        }

        pub fn path_parameter_1<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.organization_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for path_parameter_1 failed".to_string());
            self
        }

        pub fn path_parameter_2<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<u32>,
        {
            self.organization_name = value
                .try_into()
                .map_err(|_| "conversion to `u32` for path_parameter_2 failed".to_string());
            self
        }

        pub fn query_parameter_1<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.organization_name = value
                .try_into()
                .map_err(|_| "conversion to `String` for query_parameter_1 failed".to_string());
            self
        }

        pub fn query_parameter_2<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<u32>,
        {
            self.organization_name = value
                .try_into()
                .map_err(|_| "conversion to `u32` for query_parameter_2 failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: TryInto<types::ThisOperationBody>,
        {
            self.body = value
                .try_into()
                .map_err(|_| "conversion to `ThisOperationBody` for body failed".to_string());
            self
        }

        pub async fn send(self) -> Result<
            ResponseValue<types::SuccessResponseType>,
            Error<types::ErrorResponseType>,
        > {
            // ...
        }
    }
```

For more info on the `ResponseValue<T>` and `Error<E>` types, see
[progenitor_client](./progenitor-client.md).

Note that `send` methods are `async` so must be `await`ed to get the response value.

### Dropshot Paginated Operations

Dropshot defines a mechanism for pagination. If that mechanism is used for a
particular operation, Progenitor will generate an additional `stream` method on
the builder struct that produces a `Stream`. Consumers can iterate over all
items in the paginated collection without manually fetching individual pages.

Here's the signature for a typical generated method:

```rust
    impl<'a> OperationName<'a> {
        pub fn stream(
            self,
        ) -> impl futures::Stream<
	        Item = Result<types::SuccessResponseType, Error<types::ErrorResponseType>>
        > + Unpin + 'a {
            // ...
        }
    }
```

A typical consumer of this method might look like this:

```rust
    let mut stream = client.operation_name().stream();
    loop {
        match stream.try_next().await {
            Ok(Some(item)) => println!("item {:?}", item),
            Ok(None) => {
                println!("done.");
                break;
            }
            Err(_) => {
                println!("error!");
                break;
            }
        }
    }
```
