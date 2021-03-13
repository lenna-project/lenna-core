use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pipeline: Vec<ProcessorConfig>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessorConfig {
    pub id: String
}

impl Default for Config {
    fn default() -> Self {
        let resize = ProcessorConfig{
            id: "resize".to_string(),
        };
        Config {
            pipeline: vec![resize]
        }
    }
}
