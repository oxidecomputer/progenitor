// Copyright 2022 Oxide Computer Company

use indexmap::IndexMap;
use openapiv3::SecurityScheme;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::util::{sanitize, Case};
use crate::Result;

#[derive(Clone)]
pub struct SecurityRequirements(pub Vec<Vec<(Ident, Vec<String>)>>);

impl From<&Vec<IndexMap<String, Vec<String>>>> for SecurityRequirements {
    fn from(or: &Vec<IndexMap<String, Vec<String>>>) -> Self {
        SecurityRequirements(
            or.iter()
                .map(|and| {
                    and.iter()
                        .map(|(name, scopes)| {
                            let security_struct = format_ident!(
                                "{}SecurityScheme",
                                sanitize(name, Case::Pascal)
                            );
                            (security_struct, scopes.clone())
                        })
                        .collect()
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug)]
pub struct SecuritySchemeAuthenticator<'a> {
    name: &'a String,
    scheme: &'a SecurityScheme,
}

impl<'a> From<(&'a String, &'a SecurityScheme)>
    for SecuritySchemeAuthenticator<'a>
{
    fn from(args: (&'a String, &'a SecurityScheme)) -> Self {
        Self {
            name: args.0,
            scheme: args.1,
        }
    }
}

impl<'a> SecuritySchemeAuthenticator<'a> {
    pub fn new(name: &'a String, scheme: &'a SecurityScheme) -> Self {
        Self { name, scheme }
    }

    pub(crate) fn generate_tokens(&self) -> Result<TokenStream> {
        let struct_name = sanitize(self.name, Case::Pascal);
        let struct_ident = format_ident!("{}", struct_name);

        let struct_fields = self.generate_fields()?;
        let new_fn = self.generate_new_fn()?;
        let apply_fn = self.generate_apply_fn()?;

        Ok(quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_ident {
                #struct_fields
            }

            impl #struct_ident {
                #new_fn
                #apply_fn
            }

            impl ApplySecurity for #struct_ident {
                fn apply_security<'a>(
                    &'a self,
                    request: reqwest::Request
                ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<reqwest::Request, Error>> + Send + 'a>> where Self: 'a {
                    Box::pin(async {
                        self.apply(request).await
                    })
                }
            }
        })
    }

    fn generate_fields(&self) -> Result<TokenStream> {
        match self.scheme {
            SecurityScheme::APIKey { .. } => {
                unimplemented!("APIKey security schemes are not supported")
            }
            SecurityScheme::HTTP { scheme, .. } => match scheme.as_str() {
                "bearer" => Ok(quote! {
                    bearer_token: String
                }),
                "basic" => Ok(quote!(basic_token: String)),
                other => panic!(
                    "{} scheme for HTTP security scheme is not supported",
                    other
                ),
            },
            SecurityScheme::OAuth2 { .. } => {
                unimplemented!("OAuth2 security schemes are not supported")
            }
            SecurityScheme::OpenIDConnect { .. } => unimplemented!(
                "OpenIDConnect security schemes are not supported"
            ),
        }
    }

    fn generate_new_fn(&self) -> Result<TokenStream> {
        match self.scheme {
            SecurityScheme::APIKey { .. } => {
                unimplemented!("APIKey security schemes are not supported")
            }
            SecurityScheme::HTTP { scheme, .. } => match scheme.as_str() {
                "bearer" => Ok(quote! {
                    pub fn new(bearer_token: String) -> Self {
                        Self { bearer_token }
                    }
                }),
                "basic" => Ok(quote! {
                    pub fn new(username: String, password: String) -> Self {
                        Self {
                            basic_token: base64::encode(&format!("{}:{}", username, password))
                        }
                    }
                }),
                other => panic!(
                    "{} scheme for HTTP security scheme is not supported",
                    other
                ),
            },
            SecurityScheme::OAuth2 { .. } => {
                unimplemented!("OAuth2 security schemes are not supported")
            }
            SecurityScheme::OpenIDConnect { .. } => unimplemented!(
                "OpenIDConnect security schemes are not supported"
            ),
        }
    }

    fn generate_apply_fn(&self) -> Result<TokenStream> {
        match self.scheme {
            SecurityScheme::APIKey { .. } => {
                unimplemented!("APIKey security schemes are not supported")
            }
            SecurityScheme::HTTP { scheme, .. } => match scheme.as_str() {
                "bearer" => Ok(quote!(
                    pub(crate) async fn apply(
                        &self,
                        mut req: reqwest::Request,
                    ) -> Result<reqwest::Request, Error> {
                        let headers = req.headers_mut();
                        headers.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(
                                &format!("Bearer {}", self.bearer_token)
                            ).map_err(|err| {
                                Error::InvalidRequest(
                                    format!(
                                        "Failed to construct authorization header due to {:?}",
                                        err
                                    )
                                )
                            })?
                        );

                        Ok(req)
                    }
                )),
                "basic" => Ok(quote!(
                    pub(crate) async fn apply(
                        &self,
                        mut req: reqwest::Request,
                    ) -> Result<reqwest::Request, Error> {
                        let headers = req.headers_mut();
                        headers.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(
                                &format!("Basic {}", self.basic_token)
                            ).map_err(|err| {
                                Error::InvalidRequest(
                                    format!(
                                        "Failed to construct authorization header due to {:?}",
                                        err
                                    )
                                )
                            })?
                        );

                        Ok(req)
                    }
                )),
                other => panic!(
                    "{} scheme for HTTP security scheme is not supported",
                    other
                ),
            },
            SecurityScheme::OAuth2 { .. } => {
                unimplemented!("OAuth2 security schemes are not supported")
            }
            SecurityScheme::OpenIDConnect { .. } => unimplemented!(
                "OpenIDConnect security schemes are not supported"
            ),
        }
    }

    pub fn generate_security_trait() -> TokenStream {
        quote! {
            pub trait ApplySecurity: std::fmt::Debug + Send + Sync + 'static {
                fn apply_security<'a>(
                    &'a self,
                    request: reqwest::Request
                ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<reqwest::Request, Error>> + Send + 'a>> where Self: 'a;
            }
        }
    }

    pub fn generate_client_impls() -> TokenStream {
        quote! {
            impl Client {
                pub fn get_security<T>(
                    &self
                ) -> Option<&Arc<dyn ApplySecurity>> where T: ApplySecurity {
                    self.security_schemes.get(&TypeId::of::<T>())
                }

                pub fn set_security<T>(
                    &mut self,
                    scheme: T
                ) -> Option<Arc<dyn ApplySecurity>> where T: ApplySecurity {
                    self.security_schemes.insert(TypeId::of::<T>(), Arc::new(scheme))
                }

                fn has_configured_security<T>(&self) -> bool where T: ApplySecurity {
                    self.get_security::<T>().is_some()
                }

                pub async fn apply_security<T>(
                    &self,
                    request: reqwest::Request
                ) -> Result<reqwest::Request, Error> where T: ApplySecurity {
                    if let Some(scheme) = self.get_security::<T>() {
                        scheme.apply_security(request).await
                    } else {
                        Err(Error::InvalidRequest("Attempted to apply a security scheme that is not configured. This should not occur and ought to be reported as a bug.".to_string()))
                    }
                }
            }
        }
    }
}
