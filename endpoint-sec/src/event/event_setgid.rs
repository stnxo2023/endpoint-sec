//! [`EventSetgid`]

use endpoint_sec_sys::{es_event_setgid_t, uid_t};

/// A process has called `setgid()`.
#[doc(alias = "es_event_setgid_t")]
pub struct EventSetgid<'a> {
    /// Raw event
    pub(crate) raw: &'a es_event_setgid_t,
}

impl<'a> EventSetgid<'a> {
    /// Argument to the `setgid()` call.
    #[inline(always)]
    pub fn gid(&self) -> uid_t {
        self.raw.gid
    }
}

// Safety: safe to send across threads: does not contain any interior mutability nor depend on current thread state
unsafe impl Send for EventSetgid<'_> {}

impl_debug_eq_hash_with_functions!(EventSetgid<'a>; gid);
