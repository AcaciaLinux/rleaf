use crate::rleaf_sys::{cleafcore, cleafcore_getError, cleafcore_getErrorString};
use std::{error::Error, fmt};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum LeafConfigErrorClass {
    OK = 0,
    UNKNOWN = 1,
    NOTINIT = 255,
    NOCORE = 254,
    RO_CONF = 253,
    INV_CONF = 252,
}

#[derive(Debug)]
pub struct LeafCoreError {
    code: u16,
    message: String,
}

#[derive(Debug)]
pub struct LeafConfigError {
    class: LeafConfigErrorClass,
}

impl LeafCoreError {
    /// Creates a new LeafError instance from the last error set in the cleaf API
    pub fn new_from_last(core: &cleafcore) -> LeafCoreError {
        let e = LeafCoreError {
            code: unsafe { cleafcore_getError(core) },
            message: crate::util::from_c_str(unsafe { cleafcore_getErrorString(core) }),
        };
        trace!(
            "[rleaf] New LeafError: code: {}, message: \"{}\"",
            e.code,
            e.message
        );
        e
    }
}
impl Error for LeafCoreError {}
impl fmt::Display for LeafCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf error: '{}', code: {}", self.message, self.code)
    }
}
