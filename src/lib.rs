pub mod core;
pub mod plugins;

#[cfg(test)]
mod tests {
    use image::io::Reader as ImageReader;
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
        let mut img = ImageReader::open("lenna.png").unwrap().decode().unwrap();
        let config = core::config::Config::default();
        let pool = core::pool::Pool::default();
        let pipeline = core::pipeline::Pipeline::new(config, pool);
        img = pipeline.run(img);
        img.save("lenna_test_out.png").unwrap();
    }
}
