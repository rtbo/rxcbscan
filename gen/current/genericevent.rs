// Generated automatically from genericevent.xml by rs_client.py version 0.9.0.
// Do not edit!

use base;
use ffi::base::*;
use ffi::genericevent::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use std;
use std::iter::Iterator;


pub fn id() -> &'static mut base::Extension {
    unsafe {
        &mut xcb_genericevent_id
    }
}

pub const MAJOR_VERSION: u32 = 1;
pub const MINOR_VERSION: u32 = 0;

pub const QUERY_VERSION: u8 = 0;

impl base::CookieSeq for xcb_genericevent_query_version_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type QueryVersionCookie<'a> = base::Cookie<'a, xcb_genericevent_query_version_cookie_t>;

impl<'a> QueryVersionCookie<'a> {
    pub fn get_reply(self) -> Result<QueryVersionReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked { &mut err } else { std::ptr::null_mut() };
        let reply = unsafe {
            QueryVersionReply {
                ptr: xcb_genericevent_query_version_reply(
                    self.conn.get_raw_conn(),
                    self.cookie,
                    err_ptr,
                ),
            }
        };
        let checked = self.checked;
        std::mem::forget(self);

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok(reply),
            (false, true, true) => Ok(reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError { ptr: err })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type QueryVersionReply = base::Reply<xcb_genericevent_query_version_reply_t>;

impl QueryVersionReply {
    pub fn major_version(&self) -> u16 {
        unsafe { (*self.ptr).major_version }
    }
    pub fn minor_version(&self) -> u16 {
        unsafe { (*self.ptr).minor_version }
    }
}

pub fn query_version<'a>(
    c: &'a base::Connection,
    client_major_version: u16,
    client_minor_version: u16,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_genericevent_query_version(
            c.get_raw_conn(),
            client_major_version as u16,
            client_minor_version as u16,
        );
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn query_version_unchecked<'a>(
    c: &'a base::Connection,
    client_major_version: u16,
    client_minor_version: u16,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_genericevent_query_version_unchecked(
            c.get_raw_conn(),
            client_major_version as u16,
            client_minor_version as u16,
        );
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}
