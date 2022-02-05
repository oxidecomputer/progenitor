// Copyright 2021 Oxide Computer Company

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{Error, Result};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Component {
    Constant(String),
    Parameter(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PathTemplate {
    components: Vec<Component>,
}

impl PathTemplate {
    pub fn compile(&self) -> TokenStream {
        let mut fmt = String::new();
        fmt.push_str("{}");
        for c in self.components.iter() {
            fmt.push('/');
            match c {
                Component::Constant(n) => fmt.push_str(n),
                Component::Parameter(_) => fmt.push_str("{}"),
            }
        }

        let components = self.components.iter().filter_map(|component| {
            if let Component::Parameter(n) = &component {
                let param = format_ident!("{}", n);
                Some(quote! {
                    progenitor_client::encode_path(&#param.to_string())
                })
            } else {
                None
            }
        });

        quote! {
            let url = format!(#fmt, self.baseurl, #(#components,)*);
        }
    }

    pub fn names(&self) -> Vec<String> {
        self.components
            .iter()
            .filter_map(|c| match c {
                Component::Parameter(name) => Some(name.to_string()),
                Component::Constant(_) => None,
            })
            .collect()
    }
}

pub fn parse(t: &str) -> Result<PathTemplate> {
    enum State {
        Start,
        ConstantOrParameter,
        Parameter,
        ParameterSlash,
        Constant,
    }

    let mut s = State::Start;
    let mut a = String::new();
    let mut components = Vec::new();

    for c in t.chars() {
        match s {
            State::Start => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    return Err(Error::InvalidPath(
                        "path must start with a slash".to_string(),
                    ));
                }
            }
            State::ConstantOrParameter => {
                if c == '/' || c == '}' {
                    return Err(Error::InvalidPath(
                        "expected a constant or parameter".to_string(),
                    ));
                } else if c == '{' {
                    s = State::Parameter;
                } else {
                    s = State::Constant;
                    a.push(c);
                }
            }
            State::Constant => {
                if c == '/' {
                    components.push(Component::Constant(a));
                    a = String::new();
                    s = State::ConstantOrParameter;
                } else if c == '{' || c == '}' {
                    return Err(Error::InvalidPath(
                        "unexpected parameter".to_string(),
                    ));
                } else {
                    a.push(c);
                }
            }
            State::Parameter => {
                if c == '}' {
                    components.push(Component::Parameter(a));
                    a = String::new();
                    s = State::ParameterSlash;
                } else if c == '/' || c == '{' {
                    return Err(Error::InvalidPath(
                        "unexpected parameter".to_string(),
                    ));
                } else {
                    a.push(c);
                }
            }
            State::ParameterSlash => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    return Err(Error::InvalidPath(
                        "expected a slah after parameter".to_string(),
                    ));
                }
            }
        }
    }

    match s {
        State::Start => {
            return Err(Error::InvalidPath("empty path".to_string()))
        }
        State::ConstantOrParameter | State::ParameterSlash => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => {
            return Err(Error::InvalidPath(
                "unterminated parameter".to_string(),
            ))
        }
    }

    Ok(PathTemplate { components })
}

impl ToString for PathTemplate {
    fn to_string(&self) -> std::string::String {
        self.components
            .iter()
            .map(|component| match component {
                Component::Constant(s) => s.clone(),
                Component::Parameter(s) => format!("{{{}}}", s),
            })
            .fold(String::new(), |a, b| a + "/" + &b)
    }
}

#[cfg(test)]
mod test {
    use super::{parse, Component, PathTemplate};
    use anyhow::{anyhow, Context, Result};

    #[test]
    fn basic() -> Result<()> {
        let trials = vec![
            (
                "/info",
                PathTemplate {
                    components: vec![Component::Constant("info".into())],
                },
            ),
            (
                "/measure/{number}",
                PathTemplate {
                    components: vec![
                        Component::Constant("measure".into()),
                        Component::Parameter("number".into()),
                    ],
                },
            ),
            (
                "/one/{two}/three",
                PathTemplate {
                    components: vec![
                        Component::Constant("one".into()),
                        Component::Parameter("two".into()),
                        Component::Constant("three".into()),
                    ],
                },
            ),
        ];

        for (path, want) in trials.iter() {
            let t = parse(path).with_context(|| anyhow!("path {}", path))?;
            assert_eq!(&t, want);
        }

        Ok(())
    }

    #[test]
    fn names() -> Result<()> {
        let trials = vec![
            ("/info", vec![]),
            ("/measure/{number}", vec!["number".to_string()]),
            (
                "/measure/{one}/{two}/and/{three}/yeah",
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
        ];

        for (path, want) in trials.iter() {
            let t = parse(path).with_context(|| anyhow!("path {}", path))?;
            assert_eq!(&t.names(), want);
        }

        Ok(())
    }

    #[test]
    fn compile() -> Result<()> {
        let t = parse("/measure/{number}")?;
        let out = t.compile();
        let want = quote::quote! {
            let url = format!("{}/measure/{}",
                self.baseurl,
                progenitor_client::encode_path(&number.to_string()),
            );
        };
        assert_eq!(want.to_string(), out.to_string());
        Ok(())
    }
}
