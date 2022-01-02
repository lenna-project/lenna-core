//! processors pool
use super::processor::Processor;
use super::resize::Resize;

/// pool of processors
#[derive(Clone)]
pub struct Pool {
    processors: Vec<Box<dyn Processor>>,
}

impl Pool {
    pub fn add(&mut self, processor: Box<dyn Processor>) {
        self.processors.push(processor);
    }

    pub fn get(&self, id_or_name: &str) -> Option<Box<dyn Processor>> {
        for processor in self.processors.to_vec() {
            if processor.id() == id_or_name {
                return Some(processor);
            }
        }
        for processor in self.processors.to_vec() {
            if processor.name() == id_or_name {
                return Some(processor);
            }
        }
        None
    }

    pub fn ids(&self) -> Vec<String> {
        self.processors
            .iter()
            .map(|processor| processor.id())
            .collect()
    }
}

unsafe impl Send for Pool {}
unsafe impl Sync for Pool {}

impl Default for Pool {
    fn default() -> Self {
        let resize = Resize::default();
        let mut pool = Pool {
            processors: Vec::new(),
        };
        pool.add(Box::new(resize));
        pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let pool = Pool::default();
        let processor = pool.get("none");
        assert!(processor.is_none());
    }

    #[test]
    fn ids() {
        let pool = Pool::default();
        assert_eq!(pool.ids().len(), 1);
    }

    #[test]
    fn clone() {
        let pool = Pool::default();
        assert_eq!(pool.ids().len(), pool.clone().ids().len());
    }
}
