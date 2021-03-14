use super::config::{Config, ProcessorConfig};
use super::pool::Pool;
use super::processor::Processor;
use image::DynamicImage;

pub struct Pipeline {
    config: Config,
    pool: Pool,
}

impl Pipeline {
    pub fn new(config: Config, pool: Pool) -> Pipeline {
        Pipeline {
            config: config,
            pool: pool,
        }
    }

    pub fn run(&self, image: DynamicImage) -> DynamicImage {
        let mut image = image;
        for processor_config in &self.config.pipeline {
            let id = processor_config.id.clone().to_string();
            let processor = self.pool.get(&id);
            image = self.process(processor_config.clone(), image, processor);
        }
        image
    }

    pub fn process(
        &self,
        config: ProcessorConfig,
        image: DynamicImage,
        processor: Option<Box<dyn Processor>>,
    ) -> DynamicImage {
        match processor {
            Some(processor) => processor.process(config, image),
            _ => image,
        }
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            config: Config::default(),
            pool: Pool::default(),
        }
    }
}
