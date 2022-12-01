// Copyright 2022 Oxide Computer Company

use indexmap::IndexMap;
use openapiv3::{OAuth2Flow, SecurityScheme};
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
    scheme: &'a SecurityScheme,
    base_struct_ident: Ident,
}

impl<'a> From<(&'a String, &'a SecurityScheme)>
    for SecuritySchemeAuthenticator<'a>
{
    fn from(args: (&'a String, &'a SecurityScheme)) -> Self {
        Self::new(args.0, args.1)
    }
}

impl<'a> SecuritySchemeAuthenticator<'a> {
    pub fn new(name: &'a String, scheme: &'a SecurityScheme) -> Self {
        let base_struct_name = sanitize(name, Case::Pascal);
        let base_struct_ident =
            format_ident!("{}SecurityScheme", base_struct_name);

        Self {
            scheme,
            base_struct_ident,
        }
    }

    pub(crate) fn generate_tokens(&self) -> Result<TokenStream> {
        let Self {
            base_struct_ident, ..
        } = self;
        let structs = self.generate_structs()?;
        let impls = self.generate_impls()?;
        let apply_fn = self.generate_apply()?;

        Ok(quote! {
            #structs

            #impls

            #apply_fn

            impl ApplySecurity for #base_struct_ident {
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

    fn generate_structs(&self) -> Result<TokenStream> {
        let Self {
            base_struct_ident, ..
        } = self;

        match self.scheme {
            SecurityScheme::APIKey { .. } => {
                unimplemented!("APIKey security schemes are not supported")
            }
            SecurityScheme::HTTP { scheme, .. } => match scheme.as_str() {
                "bearer" => Ok(quote! {
                    #[derive(Debug)]
                    pub struct #base_struct_ident {
                        bearer_token: String
                    }
                }),
                "basic" => Ok(quote! {
                    #[derive(Debug)]
                    pub struct #base_struct_ident {
                        basic_token: String
                    }
                }),
                other => panic!(
                    "{} scheme for HTTP security scheme is not supported",
                    other
                ),
            },
            SecurityScheme::OAuth2 { flows, .. } => {
                let schema_struct = quote! {
                    #[derive(Debug)]
                    pub struct #base_struct_ident {
                        oauth_client: BasicClient,
                        refresh_url: &'static str,
                        access_token: std::sync::Arc<std::sync::RwLock<AccessToken>>,
                    }
                };

                let builders = vec![
                    &flows.implicit,
                    &flows.password,
                    &flows.client_credentials,
                    &flows.authorization_code,
                ]
                .into_iter()
                .filter_map(|v| v.as_ref())
                .map(|flow| match flow {
                    OAuth2Flow::Implicit { .. } => {
                        unimplemented!("Implicit OAuth2 is not supported")
                    }
                    OAuth2Flow::Password { .. } => {
                        let struct_ident = format_ident!(
                            "{}PasswordBuilder",
                            base_struct_ident
                        );
                        quote! {
                            #[derive(Debug)]
                            pub struct #struct_ident {
                                client_id: ClientId,
                                client_secret: ClientSecret,
                                scopes: Vec<Scope>,
                                oauth_client: BasicClient,
                                username: ResourceOwnerUsername,
                                password: ResourceOwnerPassword,
                            }
                        }
                    }
                    OAuth2Flow::ClientCredentials { .. } => {
                        let struct_ident = format_ident!(
                            "{}ClientCredentialsBuilder",
                            base_struct_ident
                        );
                        quote! {
                            #[derive(Debug)]
                            pub struct #struct_ident {
                                client_id: ClientId,
                                client_secret: ClientSecret,
                                scopes: Vec<Scope>,
                                oauth_client: BasicClient,
                            }
                        }
                    }
                    OAuth2Flow::AuthorizationCode { .. } => {
                        let struct_ident = format_ident!(
                            "{}AuthorizationCodeBuilder",
                            base_struct_ident
                        );
                        quote! {
                            #[derive(Debug)]
                            pub struct #struct_ident {
                                client_id: ClientId,
                                client_secret: ClientSecret,
                                scopes: Vec<Scope>,
                                redirect_url: RedirectUrl,
                                oauth_client: BasicClient,
                                verifier: Option<PkceCodeVerifier>,
                                csrf: Option<CsrfToken>,
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();

                Ok(quote! {
                    #schema_struct

                    #(#builders)*
                })
            }
            SecurityScheme::OpenIDConnect { .. } => unimplemented!(
                "OpenIDConnect security schemes are not supported"
            ),
        }
    }

    fn generate_impls(&self) -> Result<TokenStream> {
        let Self {
            base_struct_ident, ..
        } = self;

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
            SecurityScheme::OAuth2 { flows, .. } => {
                let impls = vec![
                    &flows.implicit,
                    &flows.password,
                    &flows.client_credentials,
                    &flows.authorization_code
                ].into_iter().filter_map(|v| v.as_ref()).map(|flow| {
                    match flow {
                        OAuth2Flow::Implicit { .. } => {
                            unimplemented!("Implicit OAuth2 is not supported")
                        },
                        OAuth2Flow::Password { refresh_url, token_url, .. } => {
                            // TODO: Should we validate scopes?

                            let builder_struct = format_ident!("{}PasswordBuilder", base_struct_ident);

                            let schema_impl = quote! {
                                impl #base_struct_ident {
                                    pub fn implicit(
                                        client_id: String,
                                        client_secret: String,
                                        username: String,
                                        password: String,
                                        scopes: Vec<String>,
                                    ) -> Result<#builder_struct, OAuthClientBuildError> {
                                        Ok(#builder_struct {
                                            client_id: ClientId::new(client_id),
                                            client_secret: ClientSecret::new(client_secret),
                                            scopes: scopes.into_iter().map(Scope::new).collect::<Vec<_>>(),
                                            oauth_client: BasicClient::new(
                                                ClientId::new(client_id),
                                                Some(ClientId::new(client_secret)),
                                                // Required by BasicClient, but is not needed
                                                AuthUrl::new(#token_url.to_string())?,
                                                Some(TokenUrl::new(#token_url.to_string())?),
                                            ),
                                            username: ResourceOwnerUsername(username),
                                            password: ResourceOwnerPassword(password),
                                        })
                                    }
                                }
                            };

                            let refresh_url = refresh_url.as_ref().unwrap_or(token_url);

                            let builder_impl = quote! {
                                impl #builder_struct {
                                    pub async fn build(self) -> Result<#base_struct_ident, OAuthClientBuildError> {
                                        let now = std::time::Instant::now();

                                        let mut req = self.oauth_client.exchange_password(
                                            self.username,
                                            self.password,
                                        );

                                        for scope in self.scopes.into_iter() {
                                            req = req.add_scope(scope);
                                        }

                                        let token = req.request_async(async_http_client).await?;
                                        let expires_at = token
                                            .expires_in()
                                            .as_ref()
                                            .and_then(|duration| now.checked_add(*duration))
                                            .ok_or(OAuthTokenError::ExpirationOutOfBounds)?;

                                        Ok(#base_struct_ident {
                                            oauth_client: self.oauth_client,
                                            refresh_url: #refresh_url,
                                            access_token: std::sync::Arc::new(std::sync::RwLock::new(AccessToken {
                                                secret: token.access_token().secret().to_string(),
                                                refresh: token.refresh_token().map(|token| token.secret().to_string()),
                                                expires_at,
                                            })),
                                        })
                                    }
                                }
                            };

                            quote! {
                                #schema_impl
                                #builder_impl
                            }
                        },
                        OAuth2Flow::ClientCredentials { token_url, refresh_url, .. } => {
                            // TODO: Should we validate scopes?

                            let builder_struct = format_ident!("{}ClientCredentialsBuilder", base_struct_ident);

                            let schema_impl = quote! {
                                impl #base_struct_ident {
                                    pub fn client_credentials(
                                        client_id: String,
                                        client_secret: String,
                                        scopes: Vec<String>,
                                    ) -> Result<#builder_struct, OAuthClientBuildError> {
                                        Ok(#builder_struct {
                                            client_id: ClientId::new(client_id),
                                            client_secret: ClientSecret::new(client_secret),
                                            scopes: scopes.into_iter().map(Scope::new).collect::<Vec<_>>(),
                                            oauth_client: BasicClient::new(
                                                ClientId::new(client_id),
                                                Some(ClientId::new(client_secret)),
                                                // Required by BasicClient, but is not needed
                                                AuthUrl::new(#token_url.to_string())?,
                                                Some(TokenUrl::new(#token_url.to_string())?),
                                            ),
                                        })
                                    }
                                }
                            };

                            let refresh_url = refresh_url.as_ref().unwrap_or(token_url);

                            let builder_impl = quote! {
                                impl #builder_struct {
                                    pub async fn build(self) -> Result<#base_struct_ident, OAuthClientBuildError> {
                                        let now = std::time::Instant::now();

                                        let mut req = self.oauth_client.exchange_client_credentials();

                                        for scope in self.scopes.into_iter() {
                                            req = req.add_scope(scope);
                                        }

                                        let token = req.request_async(async_http_client).await?;
                                        let expires_at = token
                                            .expires_in()
                                            .as_ref()
                                            .and_then(|duration| now.checked_add(*duration))
                                            .ok_or(OAuthTokenError::ExpirationOutOfBounds)?;

                                        Ok(#base_struct_ident {
                                            oauth_client: self.oauth_client,
                                            refresh_url: #refresh_url,
                                            access_token: std::sync::Arc::new(std::sync::RwLock::new(AccessToken {
                                                secret: token.access_token().secret().to_string(),
                                                refresh: token.refresh_token().map(|token| token.secret().to_string()),
                                                expires_at,
                                            })),
                                        })
                                    }
                                }
                            };

                            quote! {
                                #schema_impl
                                #builder_impl
                            }
                        },
                        OAuth2Flow::AuthorizationCode { authorization_url, token_url, refresh_url, .. } => {
                            // TODO: Should we validate scopes?

                            let builder_struct = format_ident!("{}AuthorizationCodeBuilder", base_struct_ident);

                            let schema_impl = quote! {
                                impl #base_struct_ident {
                                    pub fn authorization_code(
                                        client_id: String,
                                        client_secret: String,
                                        scopes: Vec<String>,
                                        redirect_url: String,
                                    ) -> Result<#builder_struct, OAuthClientBuildError> {
                                        Ok(#builder_struct {
                                            client_id: ClientId::new(client_id),
                                            client_secret: ClientSecret::new(client_secret),
                                            scopes: scopes.into_iter().map(Scope::new).collect::<Vec<_>>(),
                                            redirect_url: RedirectUrl::new(redirect_url)?,
                                            oauth_client: BasicClient::new(
                                                ClientId::new(client_id),
                                                Some(ClientSecret::new(client_secret)),
                                                AuthUrl::new(#authorization_url.to_string())?,
                                                Some(TokenUrl::new(#token_url.to_string())?),
                                            ),
                                            verifier: None,
                                            csrf: None,
                                        })
                                    }
                                }
                            };

                            let refresh_url = refresh_url.as_ref().unwrap_or(token_url);

                            let builder_impl = quote! {
                                impl #builder_struct {
                                    pub fn url(&mut self) -> Url {
                                        let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();
                                        let mut req = self.oauth_client.authorize_url(CsrfToken::new_random);
                                        for scope in self.scopes {
                                            req = req.add_scope(Scope::new(scope.to_string()));
                                        }

                                        let (url, csrf) = req.set_pkce_challenge(challenge).url();

                                        self.verifier = Some(verifier);
                                        self.csrf = Some(csrf);

                                        url
                                    }

                                    pub async fn build(
                                        mut self,
                                        csrf: CsrfToken,
                                        authorization_code: AuthorizationCode,
                                    ) -> Result<#base_struct_ident, OAuthClientBuildError> {
                                        if csrf.secret() != self.csrf.take().ok_or(OAuthClientBuildError::MissingCsrfToken)?.secret() {
                                            return Err(OAuthClientBuildError::InvalidCsrfToken);
                                        }

                                        let now = std::time::Instant::now();

                                        let token = self.oauth_client
                                            .exchange_code(authorization_code)
                                            .set_pkce_verifier(self.verifier.take().ok_or(OAuthClientBuildError::MissingPkceVerifier)?)
                                            .request_async(async_http_client)
                                            .await?;

                                        let expires_at = token
                                            .expires_in()
                                            .as_ref()
                                            .and_then(|duration| now.checked_add(*duration))
                                            .ok_or(OAuthTokenError::ExpirationOutOfBounds)?;

                                        Ok(#base_struct_ident {
                                            oauth_client: self.oauth_client,
                                            refresh_url: #refresh_url,
                                            access_token: std::sync::Arc::new(std::sync::RwLock::new(AccessToken {
                                                secret: token.access_token().secret().to_string(),
                                                refresh: token.refresh_token().map(|token| token.secret().to_string()),
                                                expires_at,
                                            })),
                                        })
                                    }
                                }
                            };

                            quote! {
                                #schema_impl
                                #builder_impl
                            }
                        },
                    }
                }).collect::<Vec<_>>();

                Ok(quote! {
                    #(#impls)*
                })
            }
            SecurityScheme::OpenIDConnect { .. } => unimplemented!(
                "OpenIDConnect security schemes are not supported"
            ),
        }
    }

    fn generate_apply(&self) -> Result<TokenStream> {
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
                let Self {
                    base_struct_ident, ..
                } = self;

                Ok(quote! {
                    impl #base_struct_ident {
                        async fn refresh(&self) -> Result<bool, OAuthTokenError> {
                            if let Ok(mut guard) = self.access_token.write() {
                                if let Some(token) = guard.refresh.as_ref() {
                                    let refresh = self.oauth_client.exchange_refresh_token(&RefreshToken::new(token.to_string())).request_async(async_http_client)
                                        .await?;

                                    guard.secret = refresh.access_token().secret().to_string();

                                    if let Some(new_refresh_token) = refresh.refresh_token() {
                                        guard.refresh = Some(new_refresh_token.secret().to_string());
                                    }

                                    Ok(true)
                                } else {
                                    Ok(false)
                                }
                            } else {
                                Ok(false)
                            }
                        }

                        fn token_is_expired(&self) -> bool {
                            if let Ok(guard) = self.access_token.read() {
                                guard.expires_at <= Instant::now()
                            } else {
                                // If we do not have an access token then we consider it to be expired
                                true
                            }
                        }

                        pub(crate) async fn apply(
                            &self,
                            mut req: reqwest::Request,
                        ) -> Result<reqwest::Request, Error> {
                            if self.token_is_expired() {
                                self.refresh();
                            }

                            if let Ok(guard) = self.access_token.read() {
                                let headers = req.headers_mut();
                                headers.insert(
                                    reqwest::header::AUTHORIZATION,
                                    reqwest::header::HeaderValue::from_str(
                                        &format!("Bearer {}", &guard.secret)
                                    ).map_err(|err| {
                                        Error::InvalidRequest(
                                            format!(
                                                "Failed to construct authorization header due to {:?}",
                                                err
                                            )
                                        )
                                    })?
                                );
                            }

                            Ok(req)
                        }
                    }
                })
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
