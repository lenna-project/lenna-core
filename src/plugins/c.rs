#[macro_export]
macro_rules! export_c_plugin {
    ($processor:ident) => {
        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn id() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.id()).unwrap().into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn name() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.name()).unwrap().into_raw()
        }

        #[doc(hidden)]
        #[no_mangle]
        extern "C" fn description() -> *const std::os::raw::c_char {
            let processor = $processor::default();
            std::ffi::CString::new(processor.description())
                .unwrap()
                .into_raw()
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