// Generated automatically from ge.xml by rs_client.py version 0.9.0.
// Do not edit!


use ffi::base::*;

use libc::{c_char, c_int, c_uint, c_void};
use std;


pub const XCB_GENERICEVENT_MAJOR_VERSION: u32 = 1;
pub const XCB_GENERICEVENT_MINOR_VERSION: u32 = 0;

pub const XCB_GENERICEVENT_QUERY_VERSION: u8 = 0;

#[repr(C)]
pub struct xcb_genericevent_query_version_request_t {
    pub major_opcode:         u8,
    pub minor_opcode:         u8,
    pub length:               u16,
    pub client_major_version: u16,
    pub client_minor_version: u16,
}

impl Copy for xcb_genericevent_query_version_request_t {}
impl Clone for xcb_genericevent_query_version_request_t {
    fn clone(&self) -> xcb_genericevent_query_version_request_t { *self }
}
impl ::std::fmt::Debug for xcb_genericevent_query_version_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_genericevent_query_version_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("client_major_version", &self.client_major_version)
            .field("client_minor_version", &self.client_minor_version)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_genericevent_query_version_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_genericevent_query_version_reply_t {
    pub response_type: u8,
    pub pad0:          u8,
    pub sequence:      u16,
    pub length:        u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub pad1:          [u8; 20],
}

impl Copy for xcb_genericevent_query_version_reply_t {}
impl Clone for xcb_genericevent_query_version_reply_t {
    fn clone(&self) -> xcb_genericevent_query_version_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_genericevent_query_version_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_genericevent_query_version_reply_t")
            .field("response_type", &self.response_type)
            .field("pad0", &self.pad0)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("major_version", &self.major_version)
            .field("minor_version", &self.minor_version)
            .field("pad1", &&self.pad1[..])
            .finish()
    }
}


#[link(name="xcb-ge")]
extern {

    pub static mut xcb_genericevent_id: xcb_extension_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_genericevent_query_version_reply (c:      *mut xcb_connection_t,
                                                 cookie: xcb_genericevent_query_version_cookie_t,
                                                 error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_genericevent_query_version_reply_t;

    pub fn xcb_genericevent_query_version (c:                    *mut xcb_connection_t,
                                           client_major_version: u16,
                                           client_minor_version: u16)
            -> xcb_genericevent_query_version_cookie_t;

    pub fn xcb_genericevent_query_version_unchecked (c:                    *mut xcb_connection_t,
                                                     client_major_version: u16,
                                                     client_minor_version: u16)
            -> xcb_genericevent_query_version_cookie_t;

} // extern
