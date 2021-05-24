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
            use console_error_panic_hook;
            console_error_panic_hook::set_once();

            let mut processor = $processor::default();
            let config: $crate::core::config::ProcessorConfig =
                $crate::core::config::ProcessorConfig {
                    id: processor.id(),
                    config: config.into_serde().unwrap(),
                };

            let img = $crate::io::read::read_from_data(data.to_vec()).unwrap();

            let mut lenna_img = Box::new(img);

            processor.process(config, &mut lenna_img).unwrap();

            let out_data =
                $crate::io::write::write_to_data(&lenna_img, image::ImageOutputFormat::Png)
                    .unwrap();

            out_data
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::core::processor::Processor;
    use crate::core::resize::Resize;

    #[test]
    fn resize_plugin() {
        export_wasm_plugin!(Resize);
    }
}
