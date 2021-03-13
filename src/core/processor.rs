use dyn_clone::DynClone;
use image::DynamicImage;

use super::config::ProcessorConfig;

pub trait Processor: DynClone {

    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn process(&self, config: &ProcessorConfig, image: DynamicImage) -> DynamicImage;

}

dyn_clone::clone_trait_object!(Processor);
