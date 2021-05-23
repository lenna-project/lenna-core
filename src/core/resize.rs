use super::config::ProcessorConfig;
use super::processor::{ExifProcessor, ImageProcessor, Processor};
use exif::{Exif, Field, In, Tag, Value};
use image::DynamicImage;
use serde::{Deserialize, Serialize};

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
    fn process_exif(
        &self,
        _exif: &Box<Exif>,
        exif_out: &mut Box<Vec<Field>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let width = Field {
            tag: Tag::PixelXDimension,
            ifd_num: In::PRIMARY,
            value: Value::Short(vec![self.config.width as u16]),
        };
        exif_out.push(width);
        let height = Field {
            tag: Tag::PixelYDimension,
            ifd_num: In::PRIMARY,
            value: Value::Short(vec![self.config.width as u16]),
        };
        exif_out.push(height);
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

    fn process(&mut self, config: ProcessorConfig, image: DynamicImage) -> DynamicImage {
        let config: Config = serde_json::from_value(config.config).unwrap();
        self.config = config;
        let mut img = Box::new(image);
        self.process_image(&mut img)
            .expect("Failed processing image.");
        *img
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
