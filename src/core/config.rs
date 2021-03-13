use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pipeline: Vec<ProcessorConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorConfig {
    pub id: String,
    #[serde(flatten)]
    pub config: HashMap<String, Value>,
}

impl Default for Config {
    fn default() -> Self {
        let width: Value = json!(100);
        let height: Value = json!(100);
        let mut config: HashMap<String, Value> = HashMap::new();
        config.insert("width".to_string(), width);
        config.insert("height".to_string(), height);

        let resize = ProcessorConfig {
            id: "resize".to_string(),
            config: config
        };
        Config {
            pipeline: vec![resize],
        }
    }
}
