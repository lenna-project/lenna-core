pub mod core;
pub mod plugins;

pub use crate::core::config::Config as Config;
pub use crate::core::config::ProcessorConfig as ProcessorConfig;
pub use crate::core::pipeline::Pipeline as Pipeline;
pub use crate::core::processor::Processor as Processor;
pub use crate::core::pool::Pool as Pool;

pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::plugins::PluginDeclaration =
            $crate::plugins::PluginDeclaration {
                rustc_version: $crate::RUSTC_VERSION,
                core_version: $crate::CORE_VERSION,
                register: $register,
            };
    };
}

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
