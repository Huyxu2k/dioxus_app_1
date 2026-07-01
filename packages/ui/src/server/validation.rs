use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidationError {
    pub fields: HashMap<String, String>
}

impl ValidationError {
    pub fn new() -> Self {
        Self { fields: HashMap::new() }
    }

    pub fn insert(&mut self, field: &str, message: &str){
        self.fields.insert(field.to_string(), message.to_string());
    }

    pub fn is_empty(&self) -> bool{
        self.fields.is_empty()
    }
}