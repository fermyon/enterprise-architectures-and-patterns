use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct Values {
    #[serde(default, rename = "values")]
    pub values: Vec<Value>,
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = self
            .values
            .clone()
            .into_iter()
            .map(|i| format!("{}", i))
            .collect::<Vec<String>>()
            .join("");
        write!(f, "{}", display)
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Value {
    pub message: String,
    #[serde(rename = "isFoo")]
    pub is_foo: bool,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message: {} (foo: {})\n", self.message, self.is_foo)
    }
}
