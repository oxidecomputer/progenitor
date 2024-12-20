// Copyright 2022 Oxide Computer Company

use std::collections::HashMap;

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
    pub fn compile(&self, rename: HashMap<&String, &String>, client: TokenStream) -> TokenStream {
        let mut fmt = String::new();
        fmt.push_str("{}");
        for c in self.components.iter() {
            match c {
                Component::Constant(n) => fmt.push_str(n),
                Component::Parameter(_) => fmt.push_str("{}"),
            }
        }

        let components = self.components.iter().filter_map(|component| {
            if let Component::Parameter(n) = &component {
                let param = format_ident!(
                    "{}",
                    rename
                        .get(&n)
                        .expect(&format!("missing path name mapping {}", n)),
                );
                Some(quote! {
                    encode_path(&#param.to_string())
                })
            } else {
                None
            }
        });

        quote! {
            format!(#fmt, #client.baseurl, #(#components,)*)
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

    pub fn as_wildcard(&self) -> String {
        let inner = self
            .components
            .iter()
            .map(|c| match c {
                Component::Constant(name) => name.clone(),
                Component::Parameter(_) => "[^/]*".to_string(),
            })
            .collect::<String>();
        format!("^{}$", inner)
    }

    pub fn as_wildcard_param(&self, param: &str) -> String {
        let inner = self
            .components
            .iter()
            .map(|c| match c {
                Component::Constant(name) => name.clone(),
                Component::Parameter(p) if p == param => "{}".to_string(),
                Component::Parameter(_) => ".*".to_string(),
            })
            .collect::<String>();
        format!("^{}$", inner)
    }
}

pub fn parse(t: &str) -> Result<PathTemplate> {
    enum State {
        Start,
        ConstantOrParameter,
        Parameter,
        Constant,
    }

    let mut s = State::Start;
    let mut a = String::new();
    let mut components = Vec::new();

    for c in t.chars() {
        match s {
            State::Start => {
                if c == '/' {
                    s = State::Constant;
                    a = c.to_string();
                } else {
                    return Err(Error::InvalidPath(
                        "path must start with a slash".to_string(),
                    ));
                }
            }
            State::ConstantOrParameter => {
                if c == '{' {
                    s = State::Parameter;
                    a = String::new();
                } else {
                    s = State::Constant;
                    a = c.to_string();
                }
            }
            State::Constant => {
                if c == '{' {
                    components.push(Component::Constant(a));
                    a = String::new();
                    s = State::Parameter;
                } else if c == '}' {
                    return Err(Error::InvalidPath("unexpected }".to_string()));
                } else if c != '/' || !a.ends_with('/') {
                    a.push(c);
                }
            }
            State::Parameter => {
                if c == '}' {
                    if a.contains('/') || a.contains('{') {
                        return Err(Error::InvalidPath(format!(
                            "invalid parameter name {:?}",
                            a,
                        )));
                    }
                    components.push(Component::Parameter(a));
                    a = String::new();
                    s = State::ConstantOrParameter;
                } else {
                    a.push(c);
                }
            }
        }
    }

    match s {
        State::Start => return Err(Error::InvalidPath("empty path".to_string())),
        State::ConstantOrParameter => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => return Err(Error::InvalidPath("unterminated parameter".to_string())),
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
            .fold(String::new(), |a, b| a + &b)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{parse, Component, PathTemplate};

    #[test]
    fn basic() {
        let trials = vec![
            (
                "/info",
                "/info",
                PathTemplate {
                    components: vec![Component::Constant("/info".into())],
                },
            ),
            (
                "/measure/{number}",
                "/measure/{number}",
                PathTemplate {
                    components: vec![
                        Component::Constant("/measure/".into()),
                        Component::Parameter("number".into()),
                    ],
                },
            ),
            (
                "/one/{two}/three",
                "/one/{two}/three",
                PathTemplate {
                    components: vec![
                        Component::Constant("/one/".into()),
                        Component::Parameter("two".into()),
                        Component::Constant("/three".into()),
                    ],
                },
            ),
            (
                "/{foo}-{bar}-{baz}",
                "/{foo}-{bar}-{baz}",
                PathTemplate {
                    components: vec![
                        Component::Constant("/".into()),
                        Component::Parameter("foo".into()),
                        Component::Constant("-".into()),
                        Component::Parameter("bar".into()),
                        Component::Constant("-".into()),
                        Component::Parameter("baz".into()),
                    ],
                },
            ),
            (
                "//normalise/////{adjacent}:x///slashes",
                "/normalise/{adjacent}:x/slashes",
                PathTemplate {
                    components: vec![
                        Component::Constant("/normalise/".into()),
                        Component::Parameter("adjacent".into()),
                        Component::Constant(":x/slashes".into()),
                    ],
                },
            ),
            (
                "/v1/files/{fileId}:completeUpload",
                "/v1/files/{fileId}:completeUpload",
                PathTemplate {
                    components: vec![
                        Component::Constant("/v1/files/".into()),
                        Component::Parameter("fileId".into()),
                        Component::Constant(":completeUpload".into()),
                    ],
                },
            ),
            (
                "/v1/folders/{folderId}/sessions/{sessionId}:complete",
                "/v1/folders/{folderId}/sessions/{sessionId}:complete",
                PathTemplate {
                    components: vec![
                        Component::Constant("/v1/folders/".into()),
                        Component::Parameter("folderId".into()),
                        Component::Constant("/sessions/".into()),
                        Component::Parameter("sessionId".into()),
                        Component::Constant(":complete".into()),
                    ],
                },
            ),
        ];

        for (path, expect_string, want) in trials.iter() {
            match parse(path) {
                Ok(t) => {
                    assert_eq!(&t, want);
                    assert_eq!(t.to_string().as_str(), *expect_string);
                }
                Err(e) => panic!("path {} {}", path, e),
            }
        }
    }

    #[test]
    fn names() {
        let trials = vec![
            ("/info", vec![]),
            ("/measure/{number}", vec!["number".to_string()]),
            (
                "/measure/{one}/{two}/and/{three}/yeah",
                vec!["one".to_string(), "two".to_string(), "three".to_string()],
            ),
        ];

        for (path, want) in trials.iter() {
            match parse(path) {
                Ok(t) => assert_eq!(&t.names(), want),
                Err(e) => panic!("path {} {}", path, e),
            }
        }
    }

    #[test]
    fn compile() {
        let mut rename = HashMap::new();
        let number = "number".to_string();
        rename.insert(&number, &number);
        let t = parse("/measure/{number}").unwrap();
        let out = t.compile(rename, quote::quote! { self });
        let want = quote::quote! {
            format!("{}/measure/{}",
                self.baseurl,
                encode_path(&number.to_string()),
            )
        };
        assert_eq!(want.to_string(), out.to_string());
    }

    #[test]
    fn compile2() {
        let mut rename = HashMap::new();
        let one = "one".to_string();
        let two = "two".to_string();
        let three = "three".to_string();
        rename.insert(&one, &one);
        rename.insert(&two, &two);
        rename.insert(&three, &three);
        let t = parse("/abc/def:{one}:jkl/{two}/a:{three}").unwrap();
        let out = t.compile(rename, quote::quote! { self });
        let want = quote::quote! {
            format!("{}/abc/def:{}:jkl/{}/a:{}",
                self.baseurl,
                encode_path(&one.to_string()),
                encode_path(&two.to_string()),
                encode_path(&three.to_string()),
            )
        };
        assert_eq!(want.to_string(), out.to_string());
    }
}
