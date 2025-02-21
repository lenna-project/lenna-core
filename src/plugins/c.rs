pub struct LennaImageBuffer {
    pub data: Vec<u8>,
}

/// Destroys a LennaImageBuffer instance
///
/// # Safety
///
/// - `img` must be a valid pointer to a LennaImageBuffer instance
/// - The pointer must have been created by Box::into_raw()
/// - After this call, the pointer must not be used
#[no_mangle]
pub unsafe extern "C" fn lenna_plugin_image_destroy(img: *mut LennaImageBuffer) {
    if !img.is_null() {
        drop(Box::from_raw(img));
    }
}

/// Gets the length of the image buffer data
///
/// # Safety
///
/// - `img` must be a valid pointer to a LennaImageBuffer instance
/// - The pointer must remain valid for the duration of this function call
#[no_mangle]
pub unsafe extern "C" fn lenna_plugin_image_length(img: *const LennaImageBuffer) -> libc::size_t {
    if img.is_null() {
        return 0;
    }

    (*img).data.len() as libc::size_t
}

/// Copies image data into the provided buffer
///
/// # Safety
///
/// - `img` must be a valid pointer to a LennaImageBuffer instance
/// - `buffer` must be a valid pointer to a buffer of at least `length` bytes
/// - Both pointers must remain valid for the duration of the copy
/// - The buffer must be large enough to hold the image data
#[no_mangle]
pub unsafe extern "C" fn lenna_plugin_image(
    img: *const LennaImageBuffer,
    buffer: *mut libc::c_char,
    length: libc::size_t,
) -> libc::c_int {
    if img.is_null() || buffer.is_null() {
        return -1;
    }

    let img = &*img;
    let buffer: &mut [u8] = std::slice::from_raw_parts_mut(buffer as *mut u8, length);

    if buffer.len() < img.data.len() {
        return -1;
    }

    std::ptr::copy_nonoverlapping(img.data.as_ptr(), buffer.as_mut_ptr(), img.data.len());

    img.data.len() as libc::c_int
}

/// The macro used to generate ffi c api.
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
        pub unsafe extern "C" fn lenna_plugin_process(
            data: *mut libc::c_char,
            length: libc::size_t,
        ) -> *mut $crate::plugins::c::LennaImageBuffer {
            let mut processor = $processor::default();
            let config: $crate::core::config::ProcessorConfig =
                $crate::core::config::ProcessorConfig {
                    id: processor.id(),
                    config: processor.default_config(),
                };
            let buffer: &mut [u8] =
                std::slice::from_raw_parts_mut(data as *mut u8, length as usize);
            let img = $crate::io::read::read_from_data(buffer.to_vec()).unwrap();

            let mut lenna_img = Box::new(img);

            processor.process(config, &mut lenna_img).unwrap();

            let out_data =
                $crate::io::write::write_to_data(&lenna_img, image::ImageOutputFormat::Png)
                    .unwrap();

            let buffer: $crate::plugins::c::LennaImageBuffer =
                $crate::plugins::c::LennaImageBuffer { data: out_data };

            Box::into_raw(Box::new(buffer))
        }

        #[doc(hidden)]
        #[no_mangle]
        pub unsafe extern "C" fn lenna_plugin_process_base64(
            b64img: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_char {
            use base64::engine::general_purpose::STANDARD;
            use base64::Engine;

            let mut processor = $processor::default();
            let config: $crate::core::config::ProcessorConfig =
                $crate::core::config::ProcessorConfig {
                    id: processor.id(),
                    config: processor.default_config(),
                };

            let data = std::ffi::CStr::from_ptr(b64img).to_str().unwrap();
            let buffer: Vec<u8> = STANDARD.decode(data).unwrap();
            let img = $crate::io::read::read_from_data(buffer).unwrap();

            let mut lenna_img = Box::new(img);

            processor.process(config, &mut lenna_img).unwrap();

            let out_data =
                $crate::io::write::write_to_data(&lenna_img, image::ImageOutputFormat::Png)
                    .unwrap();

            let b64img = STANDARD.encode(out_data);

            std::ffi::CString::new(b64img).unwrap().into_raw()
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
