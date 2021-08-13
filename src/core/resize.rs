use super::config::ProcessorConfig;
use super::processor::{ExifProcessor, ImageProcessor, Processor};
use crate::core::LennaImage;
use exif::{Field, In, Tag, Value};
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::include_bytes;

#[repr(C)]
#[derive(Default, Clone)]
pub struct Resize {
    config: Config,
}

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

impl ImageProcessor for Resize {
    fn process_image(
        &self,
        image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let resized = image.thumbnail(self.config.width, self.config.height);
        *image = Box::new(resized);
        Ok(())
    }
}

impl ExifProcessor for Resize {
    fn process_exif(&self, exif: &mut Box<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        let width = Field {
            tag: Tag::PixelXDimension,
            ifd_num: In::PRIMARY,
            value: Value::Short(vec![self.config.width as u16]),
        };
        exif.push(width);
        let height = Field {
            tag: Tag::PixelYDimension,
            ifd_num: In::PRIMARY,
            value: Value::Short(vec![self.config.height as u16]),
        };
        exif.push(height);
        Ok(())
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

    fn process(
        &mut self,
        config: ProcessorConfig,
        image: &mut Box<LennaImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config: Config = serde_json::from_value(config.config).unwrap();
        self.config = config;
        self.process_exif(&mut image.exif).unwrap();
        self.process_image(&mut image.image).unwrap();
        Ok(())
    }

    fn default_config(&self) -> serde_json::Value {
        serde_json::to_value(Config::default()).unwrap()
    }

    fn config_ui(&self) -> Option<String> {
        Some(include_str!("resize.vue").to_string())
    }

    fn icon(&self) -> Option<Vec<u8>> {
        let data: Vec<u8> = include_bytes!("../assets/resize.png").to_vec();
        Some(data)
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
        let ui = resize.config_ui().unwrap();
        assert_eq!(ui.starts_with("<template>"), true);
    }

    #[test]
    fn icon() {
        let resize = Resize::default();
        let icon = resize.icon();
        assert!(icon.is_some());
        assert_eq!(icon.unwrap().len(), 36408);
    }
}
