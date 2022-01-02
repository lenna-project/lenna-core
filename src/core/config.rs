//! Config functionalities
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

/// Configuration of a pipeline
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// pipeline of processor configs
    pub pipeline: Vec<ProcessorConfig>,
}

/// Processor configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessorConfig {
    /// id of processor
    pub id: String,
    /// configuration for processor
    #[serde(flatten)]
    pub config: Value,
}

impl Config {
    pub fn add(&mut self, id: String, config: Value) {
        self.remove(id.clone());
        self.pipeline.push(ProcessorConfig { id, config })
    }

    pub fn remove(&mut self, id: String) -> Option<ProcessorConfig> {
        let position = self.pipeline.iter().position(|c| c.id == id);
        match position {
            Some(position) => Some(self.pipeline.remove(position)),
            _ => None,
        }
    }

    pub fn find(&self, id: String) -> Option<&ProcessorConfig> {
        let config = self.pipeline.iter().find(|&c| c.id == id);
        match config {
            Some(config) => Some(&config),
            _ => None,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let config = Config::default();
        let cloned = config.clone();
        assert_eq!(config, cloned);
    }

    #[test]
    fn find() {
        let config = Config::default();
        let resize_config = config.find("resize".into());
        assert_eq!(resize_config, Some(config.pipeline.get(0).unwrap()));
    }

    #[test]
    fn add() {
        let mut config = Config::default();
        let values = json!({
            "bar": 42
        });
        config.add("foo".to_string(), values);
        assert_eq!(config.pipeline.len(), 2);
    }

    #[test]
    fn remove() {
        let mut config = Config::default();
        config.remove("resize".to_string());
        assert_eq!(config.pipeline.len(), 0);
    }
}
