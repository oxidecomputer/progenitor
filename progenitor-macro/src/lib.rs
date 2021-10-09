use proc_macro::TokenStream;

#[proc_macro]
pub fn progenerate(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}
