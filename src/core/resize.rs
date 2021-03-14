use image::DynamicImage;

use super::config::ProcessorConfig;
use super::processor::Processor;

#[derive(Default, Clone)]
pub struct Resize {

}

impl Processor for Resize {

    fn name(&self) -> String {
        "resize".into()
    }

    fn description(&self) -> String {
        "Plugin to resize image size.".into()
    }

    fn process(&self, config: &ProcessorConfig, image: DynamicImage) -> DynamicImage {
        let width = config.config["width"].as_u64().unwrap();
        let height = config.config["height"].as_u64().unwrap();
        image.thumbnail(width as u32, height as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let resize = Resize::default();
        assert_eq!(resize.name(), "resize");
    }
}