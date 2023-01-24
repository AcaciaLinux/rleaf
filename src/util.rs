use std::ffi::{CStr, c_char};

/// Converts a C const char* to a rust String
/// # Arguments
/// * `cstr` - The c string to convert
pub fn from_c_str(cstr: *const c_char) -> String{
    let c_str: &CStr = unsafe { CStr::from_ptr(cstr) };
    c_str.to_str().unwrap().to_owned()
}
