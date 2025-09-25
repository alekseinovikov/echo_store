use crate::{StorageAccessor, StorageError};
use async_trait::async_trait;
use serde_json::{Map, Value};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Storage {
    value: Map<String, Value>,
}

impl Storage {
    pub fn new() -> Self {
        Self { value: Map::new() }
    }
}

#[async_trait]
impl StorageAccessor for Storage {
    async fn get(&self, key: &str) -> Option<Value> {
        self.value.get(key).cloned()
    }

    async fn set(&mut self, key: &str, value: &Value) -> Result<(), StorageError> {
        self.value.insert(key.to_string(), value.clone());
        Ok(())
    }
}
