use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pipeline: Vec<ProcessorConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessorConfig {
    pub id: String,
    #[serde(flatten)]
    pub config: Value,
}

impl Default for Config {
    fn default() -> Self {
        let config = json!({
            "width": 100,
            "height": 100
        });

        let resize = ProcessorConfig {
            id: "resize".to_string(),
            config,
        };
        Config {
            pipeline: vec![resize],
        }
    }
}
