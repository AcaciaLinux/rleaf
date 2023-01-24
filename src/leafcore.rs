use std::ffi::*;
use crate::error::LeafError;

/// A struct representing a Leafcore instance
pub struct Leafcore{
    /// The raw cleaf pointer
    pub cleafcore: *mut c_void
}

impl Drop for Leafcore{
    fn drop(&mut self) {
        unsafe {
            cleafcore_delete(self.cleafcore);
        }
        crate::cleaf::remove_leaf_instance();
    }
}

impl Leafcore{
    /// Creates a new Leafcore instance
    /// # Examples
    /// ```
    /// use rleaf::leafcore::Leafcore;
    /// let leaf = Leafcore::new();
    /// ```
    pub fn new() -> Leafcore{
        crate::cleaf::add_leaf_instance();

        let newcore = Leafcore {
            cleafcore : unsafe{ cleafcore_new() }
        };

        return newcore;
    }

    /// Updates the local package list
    /// # Examples
    /// ```
    /// use rleaf::leafcore::Leafcore;
    /// let mut leaf = Leafcore::new();
    /// leaf.a_update();
    /// ```
    pub fn a_update(&mut self) -> Result<(), LeafError> {
        match unsafe { cleafcore_a_update(self.cleafcore) }{
            0 => Ok(()),
            _ => Err(LeafError::new_from_last(self))
        }
    }

    /// Installs the supplied Vec of package names
    /// # Arguments
    /// * `pkgs` - The packages to install
    /// # Examples
    /// ```
    /// use rleaf::leafcore::Leafcore;
    /// let mut leaf = Leafcore::new();
    /// leaf.a_install(&vec!["myPackage".to_string()]);
    /// ```
    pub fn a_install(&mut self, pkgs: &Vec<String>) -> Result<(), LeafError>{

        let c_strings: Vec<CString> = pkgs
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        let c_string_ptrs: Vec<*const c_char> = c_strings
            .iter()
            .map(|s| s.as_ptr())
            .collect();

        let c_string_ptrs_ptr = c_string_ptrs.as_ptr();

        match unsafe { cleafcore_a_install(self.cleafcore, c_string_ptrs.len() as u32,c_string_ptrs_ptr) }{
            0 => Ok(()),
            _ => Err(LeafError::new_from_last(self))
        }
    }

    /// Installs the supplied Vec of local package files
    /// # Arguments
    /// * `pkgs` - The package files to install
    /// # Examples
    /// ```
    /// use rleaf::leafcore::Leafcore;
    /// let mut leaf = Leafcore::new();
    /// leaf.a_install_local(&vec!["/pat/to/myPackage.lfpk".to_string()]);
    /// ```
    pub fn a_install_local(&mut self, pkgs: &Vec<String>) -> Result<(), LeafError>{

        let c_strings: Vec<CString> = pkgs
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        let c_string_ptrs: Vec<*const c_char> = c_strings
            .iter()
            .map(|s| s.as_ptr())
            .collect();

        let c_string_ptrs_ptr = c_string_ptrs.as_ptr();

        match unsafe { cleafcore_a_installLocal(self.cleafcore, c_string_ptrs.len() as u32,c_string_ptrs_ptr) }{
            0 => Ok(()),
            _ => Err(LeafError::new_from_last(self))
        }
    }

    /// Upgrades the supplied Vec of packages (empty array = all)
    /// # Arguments
    /// * `pkgs` - The packages to upgrade
    /// # Examples
    /// ```
    /// use rleaf::leafcore::Leafcore;
    /// let mut leaf = Leafcore::new();
    /// leaf.a_upgrade(&vec!["myPackage".to_string()]);
    /// ```
    pub fn a_upgrade(&mut self, pkgs: &Vec<String>) -> Result<(), LeafError>{

        let c_strings: Vec<CString> = pkgs
            .iter()
            .map(|s| CString::new(s.as_str()).unwrap())
            .collect();

        let c_string_ptrs: Vec<*const c_char> = c_strings
            .iter()
            .map(|s| s.as_ptr())
            .collect();

        let c_string_ptrs_ptr = c_string_ptrs.as_ptr();

        match unsafe { cleafcore_a_upgrade(self.cleafcore, c_string_ptrs.len() as u32,c_string_ptrs_ptr) }{
            0 => Ok(()),
            _ => Err(LeafError::new_from_last(self))
        }
    }

}

#[link(name = "cleaf")]
extern "C" {
    fn cleafcore_new() -> *mut c_void;
    fn cleafcore_delete(core: *mut c_void);
    fn cleafcore_a_update(core: *mut c_void) -> i8;
    fn cleafcore_a_install(core: *mut c_void, l_pkgs: u32, pkgs: *const *const c_char) -> i8;
    fn cleafcore_a_installLocal(core: *mut c_void, l_pkgs: u32, pkgs: *const *const c_char) -> i8;
    fn cleafcore_a_upgrade(core: *mut c_void, l_pkgs: u32, pkgs: *const *const c_char) -> i8;
}