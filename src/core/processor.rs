use crate::core::config::ProcessorConfig;
use crate::core::LennaImage;
use dyn_clone::DynClone;
use exif::Field;
use image::DynamicImage;

pub trait Processor: ImageProcessor + ExifProcessor + DynClone {
    fn id(&self) -> String {
        format!("{}_{}", self.name(), self.version())
    }
    fn name(&self) -> String;
    fn title(&self) -> String;
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
    fn author(&self) -> String;
    fn description(&self) -> String;
    fn process(
        &mut self,
        config: ProcessorConfig,
        image: &mut Box<LennaImage>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn set_config(&mut self, _config: serde_json::Value) {}
    fn default_config(&self) -> serde_json::Value;
    fn config_ui(&self) -> Option<String> {
        None
    }
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
    fn process_exif(&self, _exif: &mut Box<Vec<Field>>) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

dyn_clone::clone_trait_object!(Processor);
