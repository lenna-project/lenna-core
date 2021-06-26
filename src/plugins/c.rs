#[derive(Debug, Clone)]
#[repr(C)]
pub struct LennaImageBuffer {
    pub data: Vec<u8>,
}

#[macro_export]
macro_rules! export_c_plugin {
    ($processor:ident) => {
        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_id() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.id()).unwrap().into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_name() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.name()).unwrap().into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_title() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.title())
                .unwrap()
                .into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_version() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.version())
                .unwrap()
                .into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_author() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.author())
                .unwrap()
                .into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_description() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.description())
                .unwrap()
                .into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        pub extern "C" fn lenna_plugin_process(
            data: &[u8],
        ) -> *mut $crate::plugins::c::LennaImageBuffer {
            let mut processor = $processor::default();
            let config: $crate::core::config::ProcessorConfig =
                $crate::core::config::ProcessorConfig {
                    id: processor.id(),
                    config: processor.default_config(),
                };

            let img = $crate::io::read::read_from_data(data.to_vec()).unwrap();

            let mut lenna_img = Box::new(img);

            processor.process(config, &mut lenna_img).unwrap();

            let out_data =
                $crate::io::write::write_to_data(&lenna_img, image::ImageOutputFormat::Png)
                    .unwrap();

            let buffer: $crate::plugins::c::LennaImageBuffer =
                $crate::plugins::c::LennaImageBuffer { data: out_data };

            Box::into_raw(Box::new(buffer))
        }

        // source: https://michael-f-bryan.github.io/rust-ffi-guide/send_basic.html
        #[no_mangle]
        pub unsafe extern "C" fn lenna_plugin_image_destroy(
            img: *mut $crate::plugins::c::LennaImageBuffer,
        ) {
            if !img.is_null() {
                drop(Box::from_raw(img));
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn lenna_plugin_image_length(
            img: *const $crate::plugins::c::LennaImageBuffer,
        ) -> libc::size_t {
            if img.is_null() {
                return 0;
            }

            (&*img).data.len() as libc::size_t
        }

        #[no_mangle]
        pub unsafe extern "C" fn lenna_plugin_image(
            img: *const $crate::plugins::c::LennaImageBuffer,
            buffer: *mut libc::c_char,
            length: libc::size_t,
        ) -> libc::c_int {
            if img.is_null() || buffer.is_null() {
                return -1;
            }

            let img = &*img;
            let buffer: &mut [u8] =
                std::slice::from_raw_parts_mut(buffer as *mut u8, length as usize);

            if buffer.len() < img.data.len() {
                return -1;
            }

            std::ptr::copy_nonoverlapping(img.data.as_ptr(), buffer.as_mut_ptr(), img.data.len());

            img.data.len() as libc::c_int
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn module() {
        use crate::core::processor::Processor;
        use crate::core::resize::Resize;
        #[allow(non_camel_case_types)]
        type resize = Resize;
        export_c_plugin!(resize);
    }
}
