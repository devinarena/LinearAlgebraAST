
use std::collections::HashMap;

use crate::value::Value;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new()
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn lookup(&self, name: &String) -> Option<&Value> {
        self.values.get(name)
    }
}