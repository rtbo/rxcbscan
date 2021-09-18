use base;
use ffi::base::*;
use ffi::xinerama::*;
use ffi::xproto::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use std;
use std::iter::Iterator;
use xproto;

pub fn id() -> &'static mut base::Extension {
    unsafe { &mut xcb_xinerama_id }
}

pub const MAJOR_VERSION: u32 = 1;
pub const MINOR_VERSION: u32 = 1;

#[derive(Copy, Clone)]
pub struct ScreenInfo {
    pub base: xcb_xinerama_screen_info_t,
}

impl ScreenInfo {
    #[allow(unused_unsafe)]
    pub fn new(x_org: i16, y_org: i16, width: u16, height: u16) -> ScreenInfo {
        unsafe {
            ScreenInfo {
                base: xcb_xinerama_screen_info_t {
                    x_org: x_org,
                    y_org: y_org,
                    width: width,
                    height: height,
                },
            }
        }
    }
    pub fn x_org(&self) -> i16 {
        unsafe { self.base.x_org }
    }
    pub fn y_org(&self) -> i16 {
        unsafe { self.base.y_org }
    }
    pub fn width(&self) -> u16 {
        unsafe { self.base.width }
    }
    pub fn height(&self) -> u16 {
        unsafe { self.base.height }
    }
}

pub type ScreenInfoIterator = xcb_xinerama_screen_info_iterator_t;

impl Iterator for ScreenInfoIterator {
    type Item = ScreenInfo;
    fn next(&mut self) -> std::option::Option<ScreenInfo> {
        if self.rem == 0 {
            None
        } else {
            unsafe {
                let iter = self as *mut xcb_xinerama_screen_info_iterator_t;
                let data = (*iter).data;
                xcb_xinerama_screen_info_next(iter);
                Some(std::mem::transmute(*data))
            }
        }
    }
}

pub const QUERY_VERSION: u8 = 0;

impl base::CookieSeq for xcb_xinerama_query_version_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type QueryVersionCookie<'a> = base::Cookie<'a, xcb_xinerama_query_version_cookie_t>;

impl<'a> QueryVersionCookie<'a> {
    pub fn get_reply(self) -> Result<QueryVersionReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            QueryVersionReply {
                ptr: xcb_xinerama_query_version_reply(
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
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type QueryVersionReply = base::Reply<xcb_xinerama_query_version_reply_t>;

impl QueryVersionReply {
    pub fn major(&self) -> u16 {
        unsafe { (*self.ptr).major }
    }
    pub fn minor(&self) -> u16 {
        unsafe { (*self.ptr).minor }
    }
}

pub fn query_version<'a>(c: &'a base::Connection, major: u8, minor: u8) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_query_version(c.get_raw_conn(), major as u8, minor as u8);
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn query_version_unchecked<'a>(
    c: &'a base::Connection,
    major: u8,
    minor: u8,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie =
            xcb_xinerama_query_version_unchecked(c.get_raw_conn(), major as u8, minor as u8);
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const GET_STATE: u8 = 1;

impl base::CookieSeq for xcb_xinerama_get_state_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type GetStateCookie<'a> = base::Cookie<'a, xcb_xinerama_get_state_cookie_t>;

impl<'a> GetStateCookie<'a> {
    pub fn get_reply(self) -> Result<GetStateReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            GetStateReply {
                ptr: xcb_xinerama_get_state_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
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

pub type GetStateReply = base::Reply<xcb_xinerama_get_state_reply_t>;

impl GetStateReply {
    pub fn state(&self) -> u8 {
        unsafe { (*self.ptr).state }
    }
    pub fn window(&self) -> xproto::Window {
        unsafe { (*self.ptr).window }
    }
}

pub fn get_state<'a>(c: &'a base::Connection, window: xproto::Window) -> GetStateCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_get_state(c.get_raw_conn(), window as xcb_window_t);
        GetStateCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn get_state_unchecked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> GetStateCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_get_state_unchecked(c.get_raw_conn(), window as xcb_window_t);
        GetStateCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const GET_SCREEN_COUNT: u8 = 2;

impl base::CookieSeq for xcb_xinerama_get_screen_count_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type GetScreenCountCookie<'a> = base::Cookie<'a, xcb_xinerama_get_screen_count_cookie_t>;

impl<'a> GetScreenCountCookie<'a> {
    pub fn get_reply(self) -> Result<GetScreenCountReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            GetScreenCountReply {
                ptr: xcb_xinerama_get_screen_count_reply(
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
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type GetScreenCountReply = base::Reply<xcb_xinerama_get_screen_count_reply_t>;

impl GetScreenCountReply {
    pub fn screen_count(&self) -> u8 {
        unsafe { (*self.ptr).screen_count }
    }
    pub fn window(&self) -> xproto::Window {
        unsafe { (*self.ptr).window }
    }
}

pub fn get_screen_count<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> GetScreenCountCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_get_screen_count(c.get_raw_conn(), window as xcb_window_t);
        GetScreenCountCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn get_screen_count_unchecked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> GetScreenCountCookie<'a> {
    unsafe {
        let cookie =
            xcb_xinerama_get_screen_count_unchecked(c.get_raw_conn(), window as xcb_window_t);
        GetScreenCountCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const GET_SCREEN_SIZE: u8 = 3;

impl base::CookieSeq for xcb_xinerama_get_screen_size_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type GetScreenSizeCookie<'a> = base::Cookie<'a, xcb_xinerama_get_screen_size_cookie_t>;

impl<'a> GetScreenSizeCookie<'a> {
    pub fn get_reply(self) -> Result<GetScreenSizeReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            GetScreenSizeReply {
                ptr: xcb_xinerama_get_screen_size_reply(
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
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type GetScreenSizeReply = base::Reply<xcb_xinerama_get_screen_size_reply_t>;

impl GetScreenSizeReply {
    pub fn width(&self) -> u32 {
        unsafe { (*self.ptr).width }
    }
    pub fn height(&self) -> u32 {
        unsafe { (*self.ptr).height }
    }
    pub fn window(&self) -> xproto::Window {
        unsafe { (*self.ptr).window }
    }
    pub fn screen(&self) -> u32 {
        unsafe { (*self.ptr).screen }
    }
}

pub fn get_screen_size<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    screen: u32,
) -> GetScreenSizeCookie<'a> {
    unsafe {
        let cookie =
            xcb_xinerama_get_screen_size(c.get_raw_conn(), window as xcb_window_t, screen as u32);
        GetScreenSizeCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn get_screen_size_unchecked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    screen: u32,
) -> GetScreenSizeCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_get_screen_size_unchecked(
            c.get_raw_conn(),
            window as xcb_window_t,
            screen as u32,
        );
        GetScreenSizeCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const IS_ACTIVE: u8 = 4;

impl base::CookieSeq for xcb_xinerama_is_active_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type IsActiveCookie<'a> = base::Cookie<'a, xcb_xinerama_is_active_cookie_t>;

impl<'a> IsActiveCookie<'a> {
    pub fn get_reply(self) -> Result<IsActiveReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            IsActiveReply {
                ptr: xcb_xinerama_is_active_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
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

pub type IsActiveReply = base::Reply<xcb_xinerama_is_active_reply_t>;

impl IsActiveReply {
    pub fn state(&self) -> u32 {
        unsafe { (*self.ptr).state }
    }
}

pub fn is_active<'a>(c: &'a base::Connection) -> IsActiveCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_is_active(c.get_raw_conn());
        IsActiveCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn is_active_unchecked<'a>(c: &'a base::Connection) -> IsActiveCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_is_active_unchecked(c.get_raw_conn());
        IsActiveCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const QUERY_SCREENS: u8 = 5;

impl base::CookieSeq for xcb_xinerama_query_screens_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type QueryScreensCookie<'a> = base::Cookie<'a, xcb_xinerama_query_screens_cookie_t>;

impl<'a> QueryScreensCookie<'a> {
    pub fn get_reply(self) -> Result<QueryScreensReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            QueryScreensReply {
                ptr: xcb_xinerama_query_screens_reply(
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
            (true, false, _) => Err(base::ReplyError::GenericError(base::GenericError {
                ptr: err,
            })),
            (true, true, _) => Err(base::ReplyError::NullResponse),
            (r, e, c) => unreachable!("{:?}", (r, e, c)),
        }
    }
}

pub type QueryScreensReply = base::Reply<xcb_xinerama_query_screens_reply_t>;

impl QueryScreensReply {
    pub fn number(&self) -> u32 {
        unsafe { (*self.ptr).number }
    }
    pub fn screen_info(&self) -> ScreenInfoIterator {
        unsafe { xcb_xinerama_query_screens_screen_info_iterator(self.ptr) }
    }
}

pub fn query_screens<'a>(c: &'a base::Connection) -> QueryScreensCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_query_screens(c.get_raw_conn());
        QueryScreensCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn query_screens_unchecked<'a>(c: &'a base::Connection) -> QueryScreensCookie<'a> {
    unsafe {
        let cookie = xcb_xinerama_query_screens_unchecked(c.get_raw_conn());
        QueryScreensCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}
