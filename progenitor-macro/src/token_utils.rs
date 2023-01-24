// Copyright 2023 Oxide Computer Company

use quote::ToTokens;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{Add, Colon},
    Ident, Path, Token, TraitBoundModifier,
};

#[derive(Debug)]
pub struct TypeAndImpls {
    pub type_name: Path,
    pub colon_token: Option<Colon>,
    pub impls: Punctuated<ImplTrait, Add>,
}

impl Parse for TypeAndImpls {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let type_name: Path = input.parse()?;
        let colon_token: Option<Colon> = input.parse()?;
        let mut impls = Punctuated::default();

        if colon_token.is_some() {
            loop {
                let value: ImplTrait = input.parse()?;
                impls.push_value(value);
                if !input.peek(Token![+]) {
                    break;
                }
                let punct: Token![+] = input.parse()?;
                impls.push_punct(punct);
            }
        }

        Ok(Self {
            type_name,
            colon_token,
            impls,
        })
    }
}

impl ToTokens for TypeAndImpls {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.type_name.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.impls.to_tokens(tokens);
    }
}

#[derive(Debug)]
pub struct ImplTrait {
    pub modifier: TraitBoundModifier,
    pub impl_name: Ident,
}

impl Parse for ImplTrait {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let modifier: TraitBoundModifier = input.parse()?;
        let impl_name: Ident = input.parse()?;

        Ok(Self {
            modifier,
            impl_name,
        })
    }
}

impl ToTokens for ImplTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.modifier.to_tokens(tokens);
        self.impl_name.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::TypeAndImpls;

    use quote::{quote, ToTokens};

    #[test]
    fn test_parse_type_and_impls() {
        let input = quote! { my_crate::MyType };
        let value = syn::parse2::<TypeAndImpls>(input).unwrap();
        assert_eq!(
            value.type_name.to_token_stream().to_string(),
            "my_crate :: MyType",
        );
        assert_eq!(value.impls.len(), 0);

        let input = quote! { my_crate::MyType: ?Display + Hash };
        let value = syn::parse2::<TypeAndImpls>(input).unwrap();
        assert_eq!(
            value.type_name.to_token_stream().to_string(),
            "my_crate :: MyType",
        );
        assert_eq!(value.impls.len(), 2);
        let mut ii = value.impls.into_iter();
        assert_eq!(
            ii.next().unwrap().to_token_stream().to_string(),
            "? Display",
        );
        assert_eq!(ii.next().unwrap().to_token_stream().to_string(), "Hash",);
    }
}
