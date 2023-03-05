use crate::error::*;
use crate::leafcore::*;
pub use rleaf_sys::*;
use std::ffi::*;

pub enum CleafStringConfig {
    ROOTDIR = 0,
    PKGLISTURL = 1,
    CACHEDIR = 2,
    DOWNLOADDIR = 3,
    PACKAGESDIR = 4,
    CONFIGDIR = 5,
    INSTALLEDDIR = 6,
    HOOKSDIR = 7,
    PKGLISTPATH = 8,
    CHROOTCMD = 9,
    RUNSCRIPTSDIR = 10,
    DOWNLOADCACHE = 11,

    COUNT,
}

pub enum CleafBoolConfig {
    NOASK = 0,
    NOCLEAN = 1,
    NOPROGRESS = 2,
    FORCE = 3,
    FORCEOVERWRITE = 4,
    RUNPREINSTALL = 5,
    RUNPOSTINSTALL = 6,
    INSTALLDEPS = 7,
    CHECKREMOTEHASHUPGRADE = 8,

    COUNT,
}

pub enum CleafRedownload {
    NONE,
    SPECIFIED,
    ALL,

    COUNT,
}

impl<'a> Leafcore<'a> {
    pub fn set_str_conf(
        &mut self,
        conf: CleafStringConfig,
        value: &str,
    ) -> Result<(), LeafConfigError> {
        let c_str = CString::new(value).expect("Failed to convert string to c string");
        let res =
            unsafe { cleafconfig_setStringConfig(self.cleafcore, conf as u32, c_str.as_ptr()) };

        match res as u32 {
            CLEAFCONFIG_OK => Ok(()),
            _ => Err(LeafConfigError::new_from_u8(res)),
        }
    }

    pub fn get_str_conf(&self, conf: CleafStringConfig) -> Result<String, LeafConfigError> {
        let res = unsafe { cleafconfig_getStringConfig(self.cleafcore, conf as u32) };

        if res == std::ptr::null() {
            Err(LeafConfigError::new(LeafConfigErrorClass::INV_CONF))
        } else {
            Ok(crate::util::from_c_str(res))
        }
    }

    pub fn set_bool_conf(
        &mut self,
        conf: CleafBoolConfig,
        value: bool,
    ) -> Result<(), LeafConfigError> {
        let res = unsafe { cleafconfig_setBoolConfig(self.cleafcore, conf as u32, value as u8) };

        match res as u32 {
            CLEAFCONFIG_OK => Ok(()),
            _ => Err(LeafConfigError::new_from_u8(res)),
        }
    }

    pub fn get_bool_conf(&self, conf: CleafBoolConfig) -> Result<bool, LeafConfigError> {
        let res = unsafe { cleafconfig_getBoolConfig(self.cleafcore, conf as u32) };

        match res as u32 {
            CLEAFCONFIG_B_TRUE => Ok(true),
            CLEAFCONFIG_B_FALSE => Ok(false),
            _ => Err(LeafConfigError::new_from_u8(res)),
        }
    }

    pub fn set_redownload(&mut self, redownload: CleafRedownload) -> Result<(), LeafConfigError> {
        let res = unsafe { cleafconfig_setRedownload(self.cleafcore, redownload as u32) };

        match res as u32 {
            CLEAFCONFIG_B_TRUE => Ok(()),
            CLEAFCONFIG_B_FALSE => Ok(()),
            _ => Err(LeafConfigError::new_from_u8(res)),
        }
    }
}
