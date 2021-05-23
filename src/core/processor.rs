use dyn_clone::DynClone;
use exif::{Exif, Field};
use image::DynamicImage;

use super::config::ProcessorConfig;

pub trait Processor: ImageProcessor + ExifProcessor + DynClone {
    fn id(&self) -> String {
        format!("{}_{}", self.name(), self.version())
    }
    fn name(&self) -> String;
    fn title(&self) -> String;
    fn version(&self) -> String {
        format!("{}", env!("CARGO_PKG_VERSION"))
    }
    fn author(&self) -> String;
    fn description(&self) -> String;
    fn process(&mut self, config: ProcessorConfig, image: DynamicImage) -> DynamicImage;
    fn default_config(&self) -> serde_json::Value;
}

pub trait ImageProcessor {
    fn process_image(
        &self,
        _image: &mut Box<DynamicImage>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub trait ExifProcessor {
    fn process_exif(
        &self,
        _exif: &Box<Exif>,
        _exif_out: &mut Box<Vec<Field>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Processor);
