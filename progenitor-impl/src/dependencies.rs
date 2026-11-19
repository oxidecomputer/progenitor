// Copyright 2026 Oxide Computer Company

//! The crates generated code names, at the versions and with the features
//! that this generator is tested against.

use codespace::{Codespace, Dependency};

/// Register each crate in `names`, and apply the version and features in
/// [`pin`] to every crate registered so far, including those the typespace
/// registered for the generated types.
///
/// For a crate that the generated types name, the typespace registers no
/// version and omits features that the types need, such as the `serde`
/// feature of chrono.
///
/// # Panics
///
/// Panics if a crate in `names` has no entry in [`pin`].
pub(crate) fn register<'a>(cs: &mut Codespace, names: impl IntoIterator<Item = &'a str>) {
    let pins = names
        .into_iter()
        .map(|name| pin(name).unwrap_or_else(|| panic!("no pin for the crate `{name}`")))
        .chain(cs.dependencies().filter_map(|dep| pin(&dep.name)))
        .collect::<Vec<_>>();
    for dep in pins {
        cs.add_dependency(dep)
            .expect("the typespace registers every crate at any version");
    }

    // schemars implements `JsonSchema` for chrono and uuid types only with
    // the corresponding features enabled.
    let schemars_features = [("chrono", "chrono"), ("uuid", "uuid1")]
        .into_iter()
        .filter(|(krate, _)| cs.dependencies().any(|dep| dep.name == *krate))
        .map(|(_, feature)| feature.to_string())
        .collect::<Vec<_>>();
    if cs.dependencies().any(|dep| dep.name == "schemars") && !schemars_features.is_empty() {
        cs.add_dependency(Dependency {
            features: schemars_features,
            ..Dependency::new("schemars")
        })
        .expect("a registration at any version merges with any other");
    }
}

/// The version and features this generator is tested against for a crate
/// the generated code may name; `None` for any other crate, such as one
/// named by an `x-rust-type` extension.
fn pin(name: &str) -> Option<Dependency> {
    let dep = match name {
        // progenitor-client is released in lockstep with this crate.
        "progenitor-client" => pinned(name, env!("CARGO_PKG_VERSION")),
        "reqwest" => Dependency {
            default_features: Some(false),
            features: features(["json", "query"]),
            ..pinned(name, "0.13")
        },
        "futures" => pinned(name, "0.3"),
        "base64" => pinned(name, "0.23"),
        "rand" => pinned(name, "0.10"),
        "anyhow" => pinned(name, "1.0"),
        "clap" => Dependency {
            features: features(["string"]),
            ..pinned(name, "4.6")
        },
        "httpmock" => pinned(name, "0.8"),
        "regex" => pinned(name, "1.12"),

        // Crates that the generated types name.
        "serde" => Dependency {
            features: features(["derive"]),
            ..pinned(name, "1.0")
        },
        "serde_json" => pinned(name, "1.0"),
        "schemars" => pinned(name, "0.8"),
        "json-serde" => pinned(name, "1.1"),
        "regress" => pinned(name, "0.12"),
        "chrono" => Dependency {
            default_features: Some(false),
            features: features(["serde"]),
            ..pinned(name, "0.4")
        },
        "uuid" => Dependency {
            features: features(["serde"]),
            ..pinned(name, "1.0")
        },
        _ => return None,
    };
    Some(dep)
}

fn pinned(name: &str, version: &str) -> Dependency {
    Dependency {
        version: version.parse().expect("the pinned versions parse"),
        ..Dependency::new(name)
    }
}

fn features<const N: usize>(names: [&str; N]) -> Vec<String> {
    names.iter().map(|name| name.to_string()).collect()
}
