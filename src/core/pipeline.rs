use super::config::{Config, ProcessorConfig};
use super::pool::Pool;
use super::processor::Processor;
use crate::core::LennaImage;

pub struct Pipeline {
    config: Config,
    pool: Pool,
}

impl Pipeline {
    pub fn new(config: Config, pool: Pool) -> Pipeline {
        Pipeline { config, pool }
    }

    pub fn run(&self, image: &mut Box<LennaImage>) -> Result<(), Box<dyn std::error::Error>> {
        for processor_config in &self.config.pipeline {
            let id = processor_config.id.clone().to_string();
            let processor = self.pool.get(&id);
            self.process(processor_config.clone(), image, processor)
                .unwrap();
        }
        Ok(())
    }

    pub fn process(
        &self,
        config: ProcessorConfig,
        image: &mut Box<LennaImage>,
        processor: Option<Box<dyn Processor>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match processor {
            Some(mut processor) => processor.process(config, image),
            _ => Ok(()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let pipeline = Pipeline::default();
        let mut image = Box::new(LennaImage::default());
        pipeline.run(&mut image).unwrap();
    }
}
