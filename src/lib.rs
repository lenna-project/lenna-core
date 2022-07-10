#![doc(html_favicon_url = "https://lenna.app/favicon.ico")]
#![doc = include_str!("../README.md")]

pub mod core;
pub mod io;
pub mod plugins;

pub use crate::core::config::Config;
pub use crate::core::config::ProcessorConfig;
pub use crate::core::pipeline::Pipeline;
pub use crate::core::pool::Pool;
pub use crate::core::processor::{ExifProcessor, ImageProcessor, Processor};
pub use crate::core::LennaImage;

/// Version of lib which was used during compile.
pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Version of rust compiler which was used during compile.
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

/// The macro used to generate a plugin.
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
    use super::*;
    #[cfg(not(target_arch = "wasm32"))]
    use crate::io::read::read_from_file;
    use serde_yaml;

    #[test]
    fn pool() {
        let mut pool = core::pool::Pool::default();
        let resize = core::resize::Resize::default();
        let id = resize.id();
        let name = resize.name();
        pool.add(Box::new(resize));
        assert!(pool.get(&id).is_some());
        assert!(pool.get(&name).is_some());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn pipeline() {
        let config_file = std::fs::File::open("lenna.yml").unwrap();
        let config: core::config::Config = serde_yaml::from_reader(config_file).unwrap();
        let mut img = Box::new(read_from_file("lenna.png".to_string()).unwrap());
        let pool = core::pool::Pool::default();
        let pipeline = core::pipeline::Pipeline::new(config, pool);
        pipeline.run(&mut img).unwrap();
        img.image.save("lenna_test_out.png").unwrap();
    }
}
