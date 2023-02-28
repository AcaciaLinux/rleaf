use crate::rleaf_sys::{cleafcore, cleafcore_getError, cleafcore_getErrorString};
use std::{error::Error, fmt};

#[derive(Debug)]
pub struct LeafError{
    code: u16,
    message: String,
}
impl LeafError{
    /// Creates a new LeafError instance from the last error set in the cleaf API
    pub fn new_from_last(core: &cleafcore) -> LeafError{
        let e = LeafError { 
            code: unsafe { cleafcore_getError(core) },
            message: crate::util::from_c_str(unsafe { cleafcore_getErrorString(core) })
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
