use std::{error::Error, fmt};
use std::ffi::{c_void, c_char};
use crate::leafcore::*;

#[derive(Debug)]
pub struct LeafError{
    code: u16,
    message: String,
}
impl LeafError{
    /// Creates a new LeafError instance from the last error set in the cleaf API
    pub fn new_from_last(core: &mut Leafcore) -> LeafError{
        let e = LeafError { 
            code: unsafe { cleafcore_getError(core.cleafcore) },
            message: crate::util::from_c_str(unsafe { cleafcore_getErrorString(core.cleafcore) } )
        };
        trace!("[rleaf] New LeafError: code: {}, message: \"{}\"", e.code, e.message);
        e
    }
}
impl Error for LeafError{}
impl fmt::Display for LeafError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf error: '{}', code: {}", self.message, self.code)
    }
}

#[link(name = "cleaf")]
extern "C" {
    fn cleafcore_getError(core: *mut c_void) -> u16;
    fn cleafcore_getErrorString(core: *mut c_void) -> *mut c_char;
}
