use image::DynamicImage;

use super::config::ProcessorConfig;
use super::processor::Processor;

#[derive(Default, Clone)]
pub struct Resize {

}

impl Processor for Resize {

    fn id(&self) -> &'static str {
        "resize_0.0.1"
    }

    fn name(&self) -> &'static str {
        "resize"
    }

    fn description(&self) -> &'static str {
        "Plugin to resize image size."
    }

    fn process(&self, _config: &ProcessorConfig, image: DynamicImage) -> DynamicImage {
        image
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