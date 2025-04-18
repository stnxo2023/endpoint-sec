//! [`EventSetExtAttr`]

use std::ffi::OsStr;

use endpoint_sec_sys::es_event_setextattr_t;

use crate::File;

/// Set an extended attribute event.
#[doc(alias = "es_event_setextattr_t")]
pub struct EventSetExtAttr<'a> {
    /// Raw event
    pub(crate) raw: &'a es_event_setextattr_t,
}

impl<'a> EventSetExtAttr<'a> {
    /// The extended attribute which will be set.
    #[inline(always)]
    pub fn extattr(&self) -> &'a OsStr {
        // Safety: 'a tied to self, object obtained through ES
        unsafe { self.raw.extattr.as_os_str() }
    }

    #[inline(always)]
    /// The file for which the extended attribute will be set.
    pub fn target(&self) -> File<'a> {
        // Safety: 'a tied to self, object obtained through ES
        File::new(unsafe { self.raw.target() })
    }
}

// Safety: safe to send across threads: does not contain any interior mutability nor depend on current thread state
unsafe impl Send for EventSetExtAttr<'_> {}
// Safety: safe to share across threads: does not contain any interior mutability nor depend on current thread state
unsafe impl Sync for EventSetExtAttr<'_> {}

impl_debug_eq_hash_with_functions!(EventSetExtAttr<'a>; extattr, target);
