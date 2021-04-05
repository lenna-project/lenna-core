use super::config::ProcessorConfig;
use super::processor::Processor;
use image::DynamicImage;
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Default, Clone)]
pub struct Resize {}

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    width: u32,
    height: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            width: 400,
            height: 400,
        }
    }
}

impl Processor for Resize {
    fn name(&self) -> String {
        "resize".into()
    }

    fn title(&self) -> String {
        "Resize".into()
    }

    fn author(&self) -> String {
        "chriamue".into()
    }

    fn description(&self) -> String {
        "Plugin to resize image size.".into()
    }

    fn process(&self, config: ProcessorConfig, image: DynamicImage) -> DynamicImage {
        let config: Config = serde_json::from_value(config.config).unwrap();
        image.thumbnail(config.width, config.height)
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::to_value(Config::default()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let resize = Resize::default();
        assert_eq!(resize.name(), "resize");
        assert_eq!(resize.default_config()["width"], 400);
        assert_eq!(resize.default_config()["height"], 400);
    }
}
