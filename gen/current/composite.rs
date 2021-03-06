use base;
use ffi::base::*;
use ffi::composite::*;
use ffi::render::*;
use ffi::shape::*;
use ffi::xfixes::*;
use ffi::xproto::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use render;
use shape;
use std;
use std::iter::Iterator;
use xfixes;
use xproto;

pub fn id() -> &'static mut base::Extension {
    unsafe { &mut xcb_composite_id }
}

pub const MAJOR_VERSION: u32 = 0;
pub const MINOR_VERSION: u32 = 4;

pub type Redirect = u32;
pub const REDIRECT_AUTOMATIC: Redirect = 0x00;
pub const REDIRECT_MANUAL: Redirect = 0x01;

pub const QUERY_VERSION: u8 = 0;

impl base::CookieSeq for xcb_composite_query_version_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type QueryVersionCookie<'a> = base::Cookie<'a, xcb_composite_query_version_cookie_t>;

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
                ptr: xcb_composite_query_version_reply(
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

pub type QueryVersionReply = base::Reply<xcb_composite_query_version_reply_t>;

impl QueryVersionReply {
    pub fn major_version(&self) -> u32 {
        unsafe { (*self.ptr).major_version }
    }
    pub fn minor_version(&self) -> u32 {
        unsafe { (*self.ptr).minor_version }
    }
}

pub fn query_version<'a>(
    c: &'a base::Connection,
    client_major_version: u32,
    client_minor_version: u32,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_composite_query_version(
            c.get_raw_conn(),
            client_major_version as u32,
            client_minor_version as u32,
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
    client_major_version: u32,
    client_minor_version: u32,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_composite_query_version_unchecked(
            c.get_raw_conn(),
            client_major_version as u32,
            client_minor_version as u32,
        );
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const REDIRECT_WINDOW: u8 = 1;

pub fn redirect_window<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie =
            xcb_composite_redirect_window(c.get_raw_conn(), window as xcb_window_t, update as u8);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn redirect_window_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_redirect_window_checked(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const REDIRECT_SUBWINDOWS: u8 = 2;

pub fn redirect_subwindows<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_redirect_subwindows(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn redirect_subwindows_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_redirect_subwindows_checked(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const UNREDIRECT_WINDOW: u8 = 3;

pub fn unredirect_window<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie =
            xcb_composite_unredirect_window(c.get_raw_conn(), window as xcb_window_t, update as u8);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn unredirect_window_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_unredirect_window_checked(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const UNREDIRECT_SUBWINDOWS: u8 = 4;

pub fn unredirect_subwindows<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_unredirect_subwindows(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn unredirect_subwindows_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    update: u8,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_unredirect_subwindows_checked(
            c.get_raw_conn(),
            window as xcb_window_t,
            update as u8,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const CREATE_REGION_FROM_BORDER_CLIP: u8 = 5;

pub fn create_region_from_border_clip<'a>(
    c: &'a base::Connection,
    region: xfixes::Region,
    window: xproto::Window,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_create_region_from_border_clip(
            c.get_raw_conn(),
            region as xcb_xfixes_region_t,
            window as xcb_window_t,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn create_region_from_border_clip_checked<'a>(
    c: &'a base::Connection,
    region: xfixes::Region,
    window: xproto::Window,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_create_region_from_border_clip_checked(
            c.get_raw_conn(),
            region as xcb_xfixes_region_t,
            window as xcb_window_t,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const NAME_WINDOW_PIXMAP: u8 = 6;

pub fn name_window_pixmap<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    pixmap: xproto::Pixmap,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_name_window_pixmap(
            c.get_raw_conn(),
            window as xcb_window_t,
            pixmap as xcb_pixmap_t,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn name_window_pixmap_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
    pixmap: xproto::Pixmap,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_name_window_pixmap_checked(
            c.get_raw_conn(),
            window as xcb_window_t,
            pixmap as xcb_pixmap_t,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const GET_OVERLAY_WINDOW: u8 = 7;

impl base::CookieSeq for xcb_composite_get_overlay_window_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type GetOverlayWindowCookie<'a> = base::Cookie<'a, xcb_composite_get_overlay_window_cookie_t>;

impl<'a> GetOverlayWindowCookie<'a> {
    pub fn get_reply(self) -> Result<GetOverlayWindowReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            GetOverlayWindowReply {
                ptr: xcb_composite_get_overlay_window_reply(
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

pub type GetOverlayWindowReply = base::Reply<xcb_composite_get_overlay_window_reply_t>;

impl GetOverlayWindowReply {
    pub fn overlay_win(&self) -> xproto::Window {
        unsafe { (*self.ptr).overlay_win }
    }
}

pub fn get_overlay_window<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> GetOverlayWindowCookie<'a> {
    unsafe {
        let cookie = xcb_composite_get_overlay_window(c.get_raw_conn(), window as xcb_window_t);
        GetOverlayWindowCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn get_overlay_window_unchecked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> GetOverlayWindowCookie<'a> {
    unsafe {
        let cookie =
            xcb_composite_get_overlay_window_unchecked(c.get_raw_conn(), window as xcb_window_t);
        GetOverlayWindowCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const RELEASE_OVERLAY_WINDOW: u8 = 8;

pub fn release_overlay_window<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_composite_release_overlay_window(c.get_raw_conn(), window as xcb_window_t);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn release_overlay_window_checked<'a>(
    c: &'a base::Connection,
    window: xproto::Window,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie =
            xcb_composite_release_overlay_window_checked(c.get_raw_conn(), window as xcb_window_t);
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}
