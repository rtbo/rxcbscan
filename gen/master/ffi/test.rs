// Generated automatically from xtest.xml by rs_client.py version 0.9.0.
// Do not edit!


use ffi::base::*;
use ffi::xproto::*;

use libc::{c_char, c_int, c_uint, c_void};
use std;


pub const XCB_TEST_MAJOR_VERSION: u32 = 2;
pub const XCB_TEST_MINOR_VERSION: u32 = 2;

pub const XCB_TEST_GET_VERSION: u8 = 0;

#[repr(C)]
pub struct xcb_test_get_version_request_t {
    pub major_opcode:  u8,
    pub minor_opcode:  u8,
    pub length:        u16,
    pub major_version: u8,
    pub pad0:          u8,
    pub minor_version: u16,
}

impl Copy for xcb_test_get_version_request_t {}
impl Clone for xcb_test_get_version_request_t {
    fn clone(&self) -> xcb_test_get_version_request_t { *self }
}
impl ::std::fmt::Debug for xcb_test_get_version_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_get_version_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("major_version", &self.major_version)
            .field("pad0", &self.pad0)
            .field("minor_version", &self.minor_version)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_get_version_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_test_get_version_reply_t {
    pub response_type: u8,
    pub major_version: u8,
    pub sequence:      u16,
    pub length:        u32,
    pub minor_version: u16,
}

impl Copy for xcb_test_get_version_reply_t {}
impl Clone for xcb_test_get_version_reply_t {
    fn clone(&self) -> xcb_test_get_version_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_test_get_version_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_get_version_reply_t")
            .field("response_type", &self.response_type)
            .field("major_version", &self.major_version)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .field("minor_version", &self.minor_version)
            .finish()
    }
}

pub type xcb_test_cursor_t = u32;
pub const XCB_TEST_CURSOR_NONE   : xcb_test_cursor_t = 0x00;
pub const XCB_TEST_CURSOR_CURRENT: xcb_test_cursor_t = 0x01;

pub const XCB_TEST_COMPARE_CURSOR: u8 = 1;

#[repr(C)]
pub struct xcb_test_compare_cursor_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub window:       xcb_window_t,
    pub cursor:       xcb_cursor_t,
}

impl Copy for xcb_test_compare_cursor_request_t {}
impl Clone for xcb_test_compare_cursor_request_t {
    fn clone(&self) -> xcb_test_compare_cursor_request_t { *self }
}
impl ::std::fmt::Debug for xcb_test_compare_cursor_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_compare_cursor_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("window", &self.window)
            .field("cursor", &self.cursor)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct xcb_test_compare_cursor_cookie_t {
    pub(crate) sequence: c_uint
}

#[repr(C)]
pub struct xcb_test_compare_cursor_reply_t {
    pub response_type: u8,
    pub same:          u8,
    pub sequence:      u16,
    pub length:        u32,
}

impl Copy for xcb_test_compare_cursor_reply_t {}
impl Clone for xcb_test_compare_cursor_reply_t {
    fn clone(&self) -> xcb_test_compare_cursor_reply_t { *self }
}
impl ::std::fmt::Debug for xcb_test_compare_cursor_reply_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_compare_cursor_reply_t")
            .field("response_type", &self.response_type)
            .field("same", &self.same)
            .field("sequence", &self.sequence)
            .field("length", &self.length)
            .finish()
    }
}

pub const XCB_TEST_FAKE_INPUT: u8 = 2;

#[repr(C)]
pub struct xcb_test_fake_input_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub type_:        u8,
    pub detail:       u8,
    pub pad0:         [u8; 2],
    pub time:         u32,
    pub root:         xcb_window_t,
    pub pad1:         [u8; 8],
    pub rootX:        i16,
    pub rootY:        i16,
    pub pad2:         [u8; 7],
    pub deviceid:     u8,
}

impl Copy for xcb_test_fake_input_request_t {}
impl Clone for xcb_test_fake_input_request_t {
    fn clone(&self) -> xcb_test_fake_input_request_t { *self }
}
impl ::std::fmt::Debug for xcb_test_fake_input_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_fake_input_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("type_", &self.type_)
            .field("detail", &self.detail)
            .field("pad0", &&self.pad0[..])
            .field("time", &self.time)
            .field("root", &self.root)
            .field("pad1", &&self.pad1[..])
            .field("rootX", &self.rootX)
            .field("rootY", &self.rootY)
            .field("pad2", &&self.pad2[..])
            .field("deviceid", &self.deviceid)
            .finish()
    }
}

pub const XCB_TEST_GRAB_CONTROL: u8 = 3;

#[repr(C)]
pub struct xcb_test_grab_control_request_t {
    pub major_opcode: u8,
    pub minor_opcode: u8,
    pub length:       u16,
    pub impervious:   u8,
    pub pad0:         [u8; 3],
}

impl Copy for xcb_test_grab_control_request_t {}
impl Clone for xcb_test_grab_control_request_t {
    fn clone(&self) -> xcb_test_grab_control_request_t { *self }
}
impl ::std::fmt::Debug for xcb_test_grab_control_request_t {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("xcb_test_grab_control_request_t")
            .field("major_opcode", &self.major_opcode)
            .field("minor_opcode", &self.minor_opcode)
            .field("length", &self.length)
            .field("impervious", &self.impervious)
            .field("pad0", &&self.pad0[..])
            .finish()
    }
}


#[link(name="xcb-xtest")]
extern {

    pub static mut xcb_test_id: xcb_extension_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_test_get_version_reply (c:      *mut xcb_connection_t,
                                       cookie: xcb_test_get_version_cookie_t,
                                       error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_test_get_version_reply_t;

    pub fn xcb_test_get_version (c:             *mut xcb_connection_t,
                                 major_version: u8,
                                 minor_version: u16)
            -> xcb_test_get_version_cookie_t;

    pub fn xcb_test_get_version_unchecked (c:             *mut xcb_connection_t,
                                           major_version: u8,
                                           minor_version: u16)
            -> xcb_test_get_version_cookie_t;

    /// the returned value must be freed by the caller using libc::free().
    pub fn xcb_test_compare_cursor_reply (c:      *mut xcb_connection_t,
                                          cookie: xcb_test_compare_cursor_cookie_t,
                                          error:  *mut *mut xcb_generic_error_t)
            -> *mut xcb_test_compare_cursor_reply_t;

    pub fn xcb_test_compare_cursor (c:      *mut xcb_connection_t,
                                    window: xcb_window_t,
                                    cursor: xcb_cursor_t)
            -> xcb_test_compare_cursor_cookie_t;

    pub fn xcb_test_compare_cursor_unchecked (c:      *mut xcb_connection_t,
                                              window: xcb_window_t,
                                              cursor: xcb_cursor_t)
            -> xcb_test_compare_cursor_cookie_t;

    pub fn xcb_test_fake_input (c:        *mut xcb_connection_t,
                                type_:    u8,
                                detail:   u8,
                                time:     u32,
                                root:     xcb_window_t,
                                rootX:    i16,
                                rootY:    i16,
                                deviceid: u8)
            -> xcb_void_cookie_t;

    pub fn xcb_test_fake_input_checked (c:        *mut xcb_connection_t,
                                        type_:    u8,
                                        detail:   u8,
                                        time:     u32,
                                        root:     xcb_window_t,
                                        rootX:    i16,
                                        rootY:    i16,
                                        deviceid: u8)
            -> xcb_void_cookie_t;

    pub fn xcb_test_grab_control (c:          *mut xcb_connection_t,
                                  impervious: u8)
            -> xcb_void_cookie_t;

    pub fn xcb_test_grab_control_checked (c:          *mut xcb_connection_t,
                                          impervious: u8)
            -> xcb_void_cookie_t;

} // extern
