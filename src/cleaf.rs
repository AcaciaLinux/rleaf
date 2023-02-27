use crate::rleaf_sys::{cleaf_init, cleaf_finalize};
use std::sync::atomic::{AtomicU32, Ordering};
static LEAF_INSTANCES: AtomicU32 = AtomicU32::new(0);

/// Registers a new Leafcore instance. If no cleaf context exists, this creates one
pub fn add_leaf_instance(){
    if LEAF_INSTANCES.load(Ordering::Relaxed) == 0{
        unsafe { cleaf_init(3) };
        debug!("[rleaf] Initializing cleaf");
    }

    LEAF_INSTANCES.fetch_add(1, Ordering::Relaxed);
    trace!("[rleaf] Registering new leaf instance, {} in total now", LEAF_INSTANCES.load(Ordering::Relaxed));
}

/// Unregister a Leafcore instance. If no instances are left, this cleans up the cleaf context
pub fn remove_leaf_instance(){
    LEAF_INSTANCES.fetch_sub(1, Ordering::Relaxed);
    trace!("[rleaf] Unregistering a leaf instance, {} remaining", LEAF_INSTANCES.load(Ordering::Relaxed));

    if LEAF_INSTANCES.load(Ordering::Relaxed) == 0{
        unsafe { cleaf_finalize() };
        debug!("[rleaf] Finalizing cleaf");
    }
}
