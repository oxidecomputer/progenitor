use anyhow::{Result, bail};

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
                out.push_str(&format!("            {},\n", n));
            }
        }
        out.push_str("        );\n");
        out
    }
}

pub fn parse(t: &str) -> Result<Template> {
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
                    s = State::ConstantOrParameter;
                } else if c == '/' || c == '{' {
                    bail!("expected parameter");
                } else {
                    a.push(c);
                }
            }
        }
    }

    match s {
        State::Start => bail!("empty path"),
        State::ConstantOrParameter => (),
        State::Constant => components.push(Component::Constant(a)),
        State::Parameter => bail!("unterminated parameter"),
    }

    Ok(Template { components, })
}

#[cfg(test)]
mod test {
    use anyhow::{anyhow, Context, Result};
    use super::{parse, Template, Component};

    #[test]
    fn basic() -> Result<()> {
        let trials = vec![
            ("/info", Template {
                components: vec![
                    Component::Constant("info".into()),
                ],
            }),
            ("/measure/{number}", Template {
                components: vec![
                    Component::Constant("measure".into()),
                    Component::Parameter("number".into()),
                ],
            }),
        ];

        for (path, want) in trials.iter() {
            let t = parse(path)
                .with_context(|| anyhow!("path {}", path))?;
            assert_eq!(&t, want);
        }

        Ok(())
    }

    #[test]
    fn compile() -> Result<()> {
        let t = parse("/measure/{number}")?;
        let out = t.compile();
        let want = "        let url = format!(\"{}/measure/{}\",\
            \n            self.baseurl,\
            \n            number,\
            \n        );\n";
        assert_eq!(want, &out);
        Ok(())
    }
}
