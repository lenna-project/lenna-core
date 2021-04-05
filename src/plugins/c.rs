use crate::core::processor::Processor;
use crate::core::resize::Resize;
use std::ffi::CString;
use std::os::raw::c_char;

#[doc(hidden)]
#[no_mangle]
extern "C" fn getName() -> *const c_char {
    CString::new(Resize::default().name()).unwrap().into_raw()
}
