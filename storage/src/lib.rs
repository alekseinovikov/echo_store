use async_trait::async_trait;
use serde_json::Value;
use thiserror::Error;

mod engine;
pub mod prelude;

#[derive(Debug, Error)]
pub enum StorageError {}

#[async_trait]
pub trait StorageAccessor {
    async fn get(&self, key: &str) -> Option<Value>;
    async fn set(&mut self, key: &str, value: &Value) -> Result<(), StorageError>;
}

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::prelude::*;

    #[tokio::test]
    async fn parse_json_object() {
        let mut storage = Storage::new();
        let data = r#"{"a":1,"b":true,"c":"hello"}"#;
        let v: Value = serde_json::from_str(data).unwrap();

        storage.set("test_key", &v).await.unwrap();
        let v2 = storage.get("test_key").await.unwrap();

        assert_eq!(v, v2);

        let data2 = serde_json::to_string(&v2).unwrap();
        assert_eq!(data, data2);
    }
}
