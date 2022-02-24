# Generated Methods

Progenitor generates methods according to operations within an OpenAPI
document. Each method takes the following general form:

```rust
impl Client {
    pub async fn operation_name<'a>(
        &'a self,
	// Path parameters (if any) come first and are always mandatory
	path_parameter_1: String,
	path_parameter_2: u32,
	// Query parameters (if any) come next and may be optional
	query_parameter_1: String,
	query_parameter_2: Option<u32>,
	// A body parameter (if specified) comes last
	body: &ThisOperationBody,
    ) -> Result<
        ResponseValue<types::SuccessResponseType>,
        Error<types::ErrorResponseType>
    > {
        ..
```

For more info on the `ResponseValue<T>` and `Error<E>` types, see
[progenitor_client](./progenitor-client.md).

Note that methods are `async` so must be `await`ed to get the response.

### Dropshot Paginated Operations

The Dropshot crate defines a mechanism for pagination. If that mechanism is
used for a particular operation, Progenitor will generate an additional method
that produces a `Stream`. Consumers can iterate over all items in the paginated
collection without manually fetching individual pages.

Here's the signature for a typical generated method:

```rust
    pub fn operation_name_stream<'a>(
        &'a self,
        // Specific parameters...
        limit: Option<std::num::NonZeroU32>,
    ) -> impl futures::Stream<
	    Item = Result<types::SuccessResponseType, Error<types::ErrorResponseType>>
    > + Unpin + '_ {
        ..
```

A typical consumer of this method might look like this:

```rust
    let mut stream = client.operation_name_stream(None);
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
