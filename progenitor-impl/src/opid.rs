use std::collections::BTreeMap;

use super::{Error, Result};

/// newtype for encapsulating the combination of path and method,
/// which can uniquely identify a HTTP endpoint. The struct is
/// designed to be used as a key for map implementations
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct PathMethod {
    path: String,
    method: String,
}

impl PathMethod {
    /// Create new PathMethod. This may fail if path or method
    /// are empty.
    pub fn new(path: &str, method: &str) -> Result<Self> {
        // disallow empty path/method
        if path.is_empty() || method.is_empty() {
            return Err(Error::InternalError(
                "path and method may not be empty".to_string(),
            ));
        }

        // NOTE: In the future, we may consider checking for the proper URL path
        // format in the the future according to the RFC:
        // https://datatracker.ietf.org/doc/html/rfc3986#section-3.3

        Ok(Self {
            path: path.to_string(),
            method: method.to_string(),
        })
    }
}

#[test]
fn test_path_method() {
    // check failure conditions
    assert!(PathMethod::new("", "get").is_err());
    assert!(PathMethod::new("/foo", "").is_err());
    assert!(PathMethod::new("/foo", "get").is_ok());

    // check content is actually stored
    let pm = PathMethod::new("/foo", "get").unwrap();
    assert_eq!(pm.path, "/foo");
    assert_eq!(pm.method, "get");
}

/// Store for a one to one mapping between OAS operation IDs and
/// path/method pairs. The store
/// supports lookup in each direction.
#[derive(Default, Debug)]
pub struct OperationIds {
    opid_to_path_method: BTreeMap<String, PathMethod>,
    path_method_to_opid: BTreeMap<PathMethod, String>,
}

impl OperationIds {
    /// Find operation ID for given path and method. Returns [`None`] if
    /// no operation ID was found
    pub fn opid_for_path_method(&self, path: &str, method: &str) -> Option<&str> {
        let key = match PathMethod::new(path, method) {
            Ok(path_method) => path_method,
            Err(_) => return None,
        };

        self.path_method_to_opid.get(&key).map(|s| s.as_str())
    }

    /// Find path and method for a given operation ID. Returns [`None`] if
    /// no path/method combination was fround for the given operation ID
    pub fn path_method_for_opid(&self, operation_id: &str) -> Option<(&str, &str)> {
        match self.opid_to_path_method.get(operation_id) {
            Some(path_method) => Some((&path_method.path, &path_method.method)),
            None => None,
        }
    }

    fn remove_duplicate_char(s: &str, c: char) -> String {
        let mut r = String::new();
        r.reserve(s.len());
        let mut last_char_is_c = false;
        for l in s.chars() {
            if l != '_' || !last_char_is_c {
                r.push(l)
            }
            last_char_is_c = l == c;
        }
        r
    }

    /// Generate a new operation ID candidate for the given PathMethod, considering
    /// the number of attempts that have already been made. The number of attempts
    /// is included in the candiate name (unless it is 0), to help resolve name
    /// collisions.
    ///
    /// The operation_id names are created with the pattern
    /// `converted_path [attempt] converted_method`.
    ///
    /// For `GET /foo/bar`, this will yield an operation ID of `foo_bar_get`.
    /// It was deliberately chosen to have the method part at the end of the
    /// generated operation ID string, so that the main point of destinction
    /// is the path.
    /// This is useful when the operation ID is used to
    /// generate client method names: `foo_bar_get` and `foo_bar_post` will
    /// be listed next to each other in a method name list.
    ///
    /// For generated operation IDs that would start with a number,
    /// the character 'n' is prepended. So 'GET /1' would yield
    /// 'n1_get'
    ///
    /// The path '/' is special and replaced by 'root', so 'GET /' would yield
    /// 'root_get'.
    ///
    fn gen_operation_id(path_method: &PathMethod, attempt: u32) -> String {
        let p = match path_method.path.as_str() {
            "/" => "root".to_string(),
            path => {
                let mut p: String = path
                    .replace(|c: char| !c.is_alphanumeric(), "_")
                    .trim_matches('_')
                    .to_lowercase();
                if p.starts_with(char::is_numeric) {
                    p.insert(0, 'n');
                }
                Self::remove_duplicate_char(&p, '_')
            }
        };

        let m = path_method.method.to_lowercase();
        if attempt == 0 {
            format!("{p}_{m}")
        } else {
            format!("{p}{attempt}_{m}")
        }
    }

    /// Insert a new operation ID with with it's path and method attached.
    /// The method will fail if the operation ID, or the path and method
    /// combination already exist in this [`OperationIds`] instance.
    pub fn insert_opid_with_path_method(
        &mut self,
        operation_id: &str,
        path: &str,
        method: &str,
    ) -> Result<()> {
        let key = PathMethod::new(path, method)?;
        if self.opid_to_path_method.contains_key(operation_id) {
            return Err(Error::InternalError(format!(
                "operation id is already present: {operation_id:?}"
            )));
        }
        if self.path_method_to_opid.contains_key(&key) {
            return Err(Error::InternalError(format!(
                "the combination of path {} and method {} is already present",
                key.path, key.method
            )));
        }

        self.opid_to_path_method
            .insert(operation_id.to_string(), key.clone());
        self.path_method_to_opid
            .insert(key, operation_id.to_string());
        Ok(())
    }

    /// Insert a generated opid for the given path and method combination.
    /// The method will choose an operation ID that does not collide
    /// with pre existing operation IDs in this [`OperationIds`] instance.
    /// The method will fail if the given path and methoc combination already
    /// exists.
    pub fn insert_synthetic_opid_for_path_method(
        &mut self,
        path: &str,
        method: &str,
    ) -> Result<()> {
        let key = PathMethod::new(path, method)?;
        if self.path_method_to_opid.contains_key(&key) {
            return Err(Error::InternalError(format!(
                "operation id is already present: {key:?}"
            )));
        }

        let mut candidate;
        let mut attempt = 0;

        loop {
            candidate = Self::gen_operation_id(&key, attempt);
            attempt += 1;
            if !self.opid_to_path_method.contains_key(&candidate) {
                break;
            }
        }

        self.path_method_to_opid
            .insert(key.clone(), candidate.clone());
        self.opid_to_path_method.insert(candidate, key);
        Ok(())
    }
}

#[cfg(test)]
fn mk_pm(path: &str, method: &str) -> PathMethod {
    PathMethod::new(path, method).unwrap()
}

#[test]
fn test_operation_id_generation() {
    // all non-alphanumeric chars are replaced by '_'; leading
    // and trailing '_' are trimmed
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/foo/bar", "get"), 0),
        "foo_bar_get"
    );
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/foo/bar", "get"), 1),
        "foo_bar1_get"
    );
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/some.json", "get"), 0),
        "some_json_get"
    );

    // the root path "/" is a special case: If we'd simply replace
    // '/' with '_' and then trim '_', we'd get the empty string.
    // so we replace it with "root".
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/", "get"), 0),
        "root_get"
    );

    // numbers are a special case: because operation ids become identifiers,
    // they may not start with numbers. However, mid-word we allow numbers:
    assert_eq!(
        OperationIds::gen_operation_id(&mk_pm("/1/two/3", "get"), 0),
        "n1_two_3_get"
    );

    // path params
    assert_eq!(
        OperationIds::gen_operation_id(
            &mk_pm("/banks/{bankId}/accounts/{accountNo}/ledger", "get"),
            0
        ),
        "banks_bankid_accounts_accountno_ledger_get"
    );
    assert_eq!(
        OperationIds::gen_operation_id(
            &mk_pm("/geocoords/{longitude}/{latitude}/humidity", "get"),
            0
        ),
        "geocoords_longitude_latitude_humidity_get"
    );
}

#[test]
fn test_operation_ids() {
    let mut opids = OperationIds::default();

    // insert
    opids
        .insert_opid_with_path_method("foo_get", "/foo", "get")
        .unwrap();
    assert_eq!(opids.opid_for_path_method("/foo", "get"), Some("foo_get"));
    assert_eq!(opids.path_method_for_opid("foo_get"), Some(("/foo", "get")));
    opids
        .insert_opid_with_path_method("foo_post", "/foo", "post")
        .unwrap();
    assert_eq!(opids.opid_for_path_method("/foo", "post"), Some("foo_post"));
    assert_eq!(
        opids.path_method_for_opid("foo_post"),
        Some(("/foo", "post"))
    );

    // insert must fail because of collision with operation id
    assert!(opids
        .insert_opid_with_path_method("foo_get", "/bar", "get")
        .is_err());

    // insert must fail because of collision with path and method
    assert!(opids
        .insert_opid_with_path_method("bar_get", "/foo", "get")
        .is_err());

    // now check we can create synthetic operation ids:
    assert!(opids
        .insert_synthetic_opid_for_path_method("/bar", "get")
        .is_ok());
    assert_eq!(opids.opid_for_path_method("/bar", "get"), Some("bar_get"));
    assert_eq!(opids.path_method_for_opid("bar_get"), Some(("/bar", "get")));
    assert!(opids
        .insert_synthetic_opid_for_path_method("/bar", "post")
        .is_ok());
    assert_eq!(opids.opid_for_path_method("/bar", "post"), Some("bar_post"));
    assert_eq!(
        opids.path_method_for_opid("bar_post"),
        Some(("/bar", "post"))
    );

    // test collisions.
    // we're going to collide with foo_bar_get
    opids
        .insert_opid_with_path_method("foo_bar_get", "/foobar", "get")
        .unwrap();
    opids
        .insert_synthetic_opid_for_path_method("/foo/bar", "get")
        .unwrap();
    assert_eq!(
        opids.opid_for_path_method("/foo/bar", "get"),
        Some("foo_bar1_get")
    );
    assert_eq!(
        opids.path_method_for_opid("foo_bar1_get"),
        Some(("/foo/bar", "get"))
    );
}
