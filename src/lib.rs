pub mod core;
pub mod plugins;

#[cfg(test)]
mod tests {
    use image::io::Reader as ImageReader;
    use serde_yaml;
    use crate::core;
    use crate::core::processor::Processor;

    #[test]
    fn pool() {
        let mut pool = core::pool::Pool::default();
        let resize = core::resize::Resize::default();
        let id = resize.id();
        let name = resize.name();
        pool.add(Box::new(resize));
        assert!(pool.get(id).is_some());
        assert!(pool.get(name).is_some());
    }

    #[test]
    fn pipeline() {
        let config_file = std::fs::File::open("lenna.yml").unwrap();
        let config: core::config::Config = serde_yaml::from_reader(config_file).unwrap();
        let mut img = ImageReader::open("lenna.png").unwrap().decode().unwrap();
        let pool = core::pool::Pool::default();
        let pipeline = core::pipeline::Pipeline::new(config, pool);
        img = pipeline.run(img);
        img.save("lenna_test_out.png").unwrap();
    }
}
