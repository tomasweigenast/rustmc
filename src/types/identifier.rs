use lazy_static::lazy_static;
use regex::{Error, Regex};

#[derive(Debug)]
pub struct Identifier {
    namespace: String,
    value: String,
}

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"[a-z0-9.-_]").unwrap();
    static ref VALUE_REGEX: Regex = Regex::new(r"[a-z0-9.-_/]").unwrap();
}

impl Identifier {
    pub fn new(namespace: &str, value: &str) -> Result<Identifier, Error> {
        if !NAMESPACE_REGEX.is_match(namespace) {
            return Err(Error::Syntax(
                "namespace does not match regex [a-z0-9.-_]".to_string(),
            ));
        }

        if !VALUE_REGEX.is_match(value) {
            return Err(Error::Syntax(
                "value does not match regex [a-z0-9.-_/]".to_string(),
            ));
        }

        Ok(Identifier {
            namespace: namespace.to_string(),
            value: value.to_string(),
        })
    }

    pub fn namespace(&self) -> &String {
        return &self.namespace;
    }

    pub fn value(&self) -> &String {
        return &self.value;
    }

    /// Formats the Identifier in the form: namespace:value
    pub fn format(&self) -> String {
        return format!("{}:{}", self.namespace, self.value);
    }
}
