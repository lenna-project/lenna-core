use dyn_clone::DynClone;
use image::DynamicImage;

use super::config::ProcessorConfig;

pub trait Processor: DynClone {
    fn id(&self) -> String {
        format!("{}_{}", self.name(), env!("CARGO_PKG_VERSION"))
    }
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn process(&self, config: &ProcessorConfig, image: DynamicImage) -> DynamicImage;
}

dyn_clone::clone_trait_object!(Processor);
