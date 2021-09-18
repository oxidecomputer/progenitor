use anyhow::{anyhow, bail, Context, Result};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Component {
    Constant(String),
    Parameter(String),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Template {
    components: Vec<Component>,
}

impl Template {
    pub fn compile(&self) -> String {
        let mut out = "        let url = format!(\"{}".to_string();
        for c in self.components.iter() {
            out.push('/');
            match c {
                Component::Constant(n) => out.push_str(n),
                Component::Parameter(_) => out.push_str("{}"),
            }
        }
        out.push_str("\",\n");
        out.push_str("            self.baseurl,\n");
        for c in self.components.iter() {
            if let Component::Parameter(n) = &c {
                out.push_str(&format!(
                    "            \
                    progenitor_support::encode_path(&{}.to_string()),\n",
                    n
                ));
            }
        }
        out.push_str("        );\n");
        out
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

pub fn parse(t: &str) -> Result<Template> {
    parse_inner(t)
        .with_context(|| anyhow!("parse failure for template {:?}", t))
}

fn parse_inner(t: &str) -> Result<Template> {
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
                    bail!("path must start with a slash");
                }
            }
            State::ConstantOrParameter => {
                if c == '/' || c == '}' {
                    bail!("expected a constant or parameter");
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
                    bail!("unexpected parameter");
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
                    bail!("expected parameter");
                } else {
                    a.push(c);
                }
            }
            State::ParameterSlash => {
                if c == '/' {
                    s = State::ConstantOrParameter;
                } else {
                    bail!("expected a slash after parameter");
                }
            }
        }
    }

    match s {
        State::Start => bail!("empty path"),
        State::ConstantOrParameter | State::ParameterSlash => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => bail!("unterminated parameter"),
    }

    Ok(Template { components })
}

#[cfg(test)]
mod test {
    use super::{parse, Component, Template};
    use anyhow::{anyhow, Context, Result};

    #[test]
    fn basic() -> Result<()> {
        let trials = vec![
            (
                "/info",
                Template {
                    components: vec![Component::Constant("info".into())],
                },
            ),
            (
                "/measure/{number}",
                Template {
                    components: vec![
                        Component::Constant("measure".into()),
                        Component::Parameter("number".into()),
                    ],
                },
            ),
            (
                "/one/{two}/three",
                Template {
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
        let want = "        let url = format!(\"{}/measure/{}\",\
            \n            self.baseurl,\
            \n            progenitor_support::encode_path(&number.to_string()),\
            \n        );\n";
        assert_eq!(want, &out);
        Ok(())
    }
}
