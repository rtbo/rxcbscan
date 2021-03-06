use base;
use ffi::base::*;
use ffi::test::*;
use ffi::xproto::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use std;
use std::iter::Iterator;
use xproto;

pub fn id() -> &'static mut base::Extension {
    unsafe { &mut xcb_test_id }
}

pub const MAJOR_VERSION: u32 = 2;
pub const MINOR_VERSION: u32 = 2;

pub type Cursor = u32;
pub const CURSOR_NONE: Cursor = 0x00;
pub const CURSOR_CURRENT: Cursor = 0x01;

pub const GET_VERSION: u8 = 0;

impl base::CookieSeq for xcb_test_get_version_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type GetVersionCookie<'a> = base::Cookie<'a, xcb_test_get_version_cookie_t>;

impl<'a> GetVersionCookie<'a> {
    pub fn get_reply(self) -> Result<GetVersionReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            GetVersionReply {
                ptr: xcb_test_get_version_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
            }
        };
        let checked = self.checked;
        std::mem::forget(self);

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok(reply),
            (false, true, true) => Ok(reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type GetVersionReply = base::Reply<xcb_test_get_version_reply_t>;

impl GetVersionReply {
    pub fn major_version(&self) -> u8 {
        unsafe { (*self.ptr).major_version }
    }
    pub fn minor_version(&self) -> u16 {
        unsafe { (*self.ptr).minor_version }
    }
}

pub fn get_version<'a>(
    c: &'a base::Connection,
    major_version: u8,
    minor_version: u16,
) -> GetVersionCookie<'a> {
    unsafe {
        let cookie =
            xcb_test_get_version(c.get_raw_conn(), major_version as u8, minor_version as u16);
        GetVersionCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn get_version_unchecked<'a>(
    c: &'a base::Connection,
    major_version: u8,
    minor_version: u16,
) -> GetVersionCookie<'a> {
    unsafe {
        let cookie = xcb_test_get_version_unchecked(
            c.get_raw_conn(),
            major_version as u8,
            minor_version as u16,
        );
        GetVersionCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const COMPARE_CURSOR: u8 = 1;

impl base::CookieSeq for xcb_test_compare_cursor_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type CompareCursorCookie<'a> = base::Cookie<'a, xcb_test_compare_cursor_cookie_t>;

impl<'a> CompareCursorCookie<'a> {
    pub fn get_reply(self) -> Result<CompareCursorReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            CompareCursorReply {
                ptr: xcb_test_compare_cursor_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
            }
        };
        let checked = self.checked;
        std::mem::forget(self);

        match (reply.ptr.is_null(), err.is_null(), checked) {
            (false, _, false) => Ok(reply),
            (false, true, true) => Ok(reply),
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type CompareCursorReply = base::Reply<xcb_test_compare_cursor_reply_t>;

impl CompareCursorReply {
    pub fn same(&self) -> bool {
        unsafe { (*self.ptr).same != 0 }
    }
}

pub fn compare_cursor<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    cursor: xproto::Cursor,
) -> CompareCursorCookie<'a> {
    unsafe {
        let cookie = xcb_test_compare_cursor(
            c.get_raw_conn(),
            window as xcb_window_t,
            cursor as xcb_cursor_t,
        );
        CompareCursorCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn compare_cursor_unchecked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    cursor: xproto::Cursor,
) -> CompareCursorCookie<'a> {
    unsafe {
        let cookie = xcb_test_compare_cursor_unchecked(
            c.get_raw_conn(),
            window as xcb_window_t,
            cursor as xcb_cursor_t,
        );
        CompareCursorCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const FAKE_INPUT: u8 = 2;

pub fn fake_input<'a>(
    c: &'a base::Connection,
    type_: u8,
    detail: u8,
    time: u32,
    root: xproto::Window,
    root_x: i16,
    root_y: i16,
    deviceid: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_test_fake_input(
            c.get_raw_conn(),
            type_ as u8,
            detail as u8,
            time as u32,
            root as xcb_window_t,
            root_x as i16,
            root_y as i16,
            deviceid as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn fake_input_checked<'a>(
    c: &'a base::Connection,
    type_: u8,
    detail: u8,
    time: u32,
    root: xproto::Window,
    root_x: i16,
    root_y: i16,
    deviceid: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_test_fake_input_checked(
            c.get_raw_conn(),
            type_ as u8,
            detail as u8,
            time as u32,
            root as xcb_window_t,
            root_x as i16,
            root_y as i16,
            deviceid as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const GRAB_CONTROL: u8 = 3;

pub fn grab_control<'a>(c: &'a base::Connection, impervious: bool) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_test_grab_control(c.get_raw_conn(), impervious as u8);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn grab_control_checked<'a>(c: &'a base::Connection, impervious: bool) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_test_grab_control_checked(c.get_raw_conn(), impervious as u8);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}
