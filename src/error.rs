use crate::rleaf_sys::{cleafcore, cleafcore_getError, cleafcore_getErrorString};
use std::{error::Error, fmt};

pub trait LeafError{
    fn what() -> String;
}

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

impl LeafConfigError {
    pub fn new(class: LeafConfigErrorClass) -> LeafConfigError {
        let e = LeafConfigError { class: class };
        trace!("[cleaf] New LeafConfigError: {}", e.class.to_string());
        e
    }

    pub fn new_from_u8(code: u8) -> LeafConfigError {
        let e = LeafConfigError {
            class: LeafConfigErrorClass::new_from_u8(code),
        };
        trace!("[cleaf] New LeafConfigError: {}", e.class.to_string());
        e
    }
}
impl Error for LeafConfigError {}
impl fmt::Display for LeafConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leaf config error: {}", self.class.to_string())
    }
}

impl LeafConfigErrorClass {
    pub fn new_from_u8(code: u8) -> LeafConfigErrorClass {
        match code {
            0 => Self::OK,
            255 => Self::NOTINIT,
            254 => Self::NOCORE,
            253 => Self::RO_CONF,
            252 => Self::INV_CONF,
            _ => Self::UNKNOWN,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::OK => "Ok",
            Self::NOTINIT => "Not initialized",
            Self::NOCORE => "No core supplied",
            Self::RO_CONF => "Tried to write read only config",
            Self::INV_CONF => "Invalid config",
            Self::UNKNOWN => "Unknown",
        }
        .to_owned()
    }
}
