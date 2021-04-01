#![cfg(target_arch = "wasm32")]

#[macro_export]
macro_rules! export_wasm_plugin {
    ($processor:ident) => {
        use wasm_bindgen::prelude::*;

        #[doc(hidden)]
        #[wasm_bindgen(js_name = defaultConfig)]
        pub fn default_config() -> wasm_bindgen::JsValue {
            let processor = $processor::default();
            wasm_bindgen::JsValue::from_serde(&processor.default_config()).unwrap()
        }

        #[doc(hidden)]
        #[wasm_bindgen(js_name = id)]
        pub fn id() -> String {
            let processor = $processor::default();
            processor.id()
        }

        #[doc(hidden)]
        #[wasm_bindgen(js_name = name)]
        pub fn name() -> String {
            let processor = $processor::default();
            processor.name()
        }

        #[doc(hidden)]
        #[wasm_bindgen(js_name = description)]
        pub fn description() -> String {
            let processor = $processor::default();
            processor.description()
        }

        #[doc(hidden)]
        #[wasm_bindgen(js_name = process)]
        pub fn process(config: wasm_bindgen::JsValue, data: &[u8]) -> Vec<u8> {
            use std::io::{Read, Seek};
            
            let processor = $processor::default();
            let mut config: $crate::core::config::ProcessorConfig = $crate::core::config::ProcessorConfig {
                id: processor.id(),
                config: config.into_serde().unwrap(),
            };

            let img = image::load_from_memory(&data).unwrap();
            let img = processor.process(config, img);
            let mut c = std::io::Cursor::new(Vec::new());
            match img.write_to(&mut c, image::ImageOutputFormat::Png) {
                Ok(_) => (),
                Err(_) => return data.to_vec(),
            };
            c.seek(std::io::SeekFrom::Start(0)).unwrap();
            let mut out_data = Vec::new();
            c.read_to_end(&mut out_data).unwrap();
            out_data
        }
    };
}
