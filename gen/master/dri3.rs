use base;
use ffi::base::*;
use ffi::dri3::*;
use ffi::xproto::*;
use libc::{self, c_char, c_int, c_uint, c_void};
use std;
use std::iter::Iterator;
use xproto;

pub fn id() -> &'static mut base::Extension {
    unsafe { &mut xcb_dri3_id }
}

pub const MAJOR_VERSION: u32 = 1;
pub const MINOR_VERSION: u32 = 0;

pub const QUERY_VERSION: u8 = 0;

impl base::CookieSeq for xcb_dri3_query_version_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type QueryVersionCookie<'a> = base::Cookie<'a, xcb_dri3_query_version_cookie_t>;

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
                ptr: xcb_dri3_query_version_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
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

pub type QueryVersionReply = base::Reply<xcb_dri3_query_version_reply_t>;

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
    major_version: u32,
    minor_version: u32,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie =
            xcb_dri3_query_version(c.get_raw_conn(), major_version as u32, minor_version as u32);
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn query_version_unchecked<'a>(
    c: &'a base::Connection,
    major_version: u32,
    minor_version: u32,
) -> QueryVersionCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_query_version_unchecked(
            c.get_raw_conn(),
            major_version as u32,
            minor_version as u32,
        );
        QueryVersionCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const OPEN: u8 = 1;

impl base::CookieSeq for xcb_dri3_open_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type OpenCookie<'a> = base::Cookie<'a, xcb_dri3_open_cookie_t>;

impl<'a> OpenCookie<'a> {
    pub fn get_reply(self) -> Result<OpenReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            OpenReply {
                ptr: xcb_dri3_open_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
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

pub type OpenReply = base::Reply<xcb_dri3_open_reply_t>;

impl OpenReply {
    pub fn device_fds(&self, c: &base::Connection) -> &[i32] {
        unsafe {
            let nfd = (*self.ptr).nfd as usize;
            let ptr = xcb_dri3_open_reply_fds(c.get_raw_conn(), self.ptr);
            std::slice::from_raw_parts(ptr, nfd)
        }
    }
}

pub fn open<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    provider: u32,
) -> OpenCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_open(
            c.get_raw_conn(),
            drawable as xcb_drawable_t,
            provider as u32,
        );
        OpenCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn open_unchecked<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    provider: u32,
) -> OpenCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_open_unchecked(
            c.get_raw_conn(),
            drawable as xcb_drawable_t,
            provider as u32,
        );
        OpenCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const PIXMAP_FROM_BUFFER: u8 = 2;

pub fn pixmap_from_buffer<'a>(
    c: &'a base::Connection,
    pixmap: xproto::Pixmap,
    drawable: xproto::Drawable,
    size: u32,
    width: u16,
    height: u16,
    stride: u16,
    depth: u8,
    bpp: u8,
    pixmap_fd: i32,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_pixmap_from_buffer(
            c.get_raw_conn(),
            pixmap as xcb_pixmap_t,
            drawable as xcb_drawable_t,
            size as u32,
            width as u16,
            height as u16,
            stride as u16,
            depth as u8,
            bpp as u8,
            pixmap_fd as i32,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn pixmap_from_buffer_checked<'a>(
    c: &'a base::Connection,
    pixmap: xproto::Pixmap,
    drawable: xproto::Drawable,
    size: u32,
    width: u16,
    height: u16,
    stride: u16,
    depth: u8,
    bpp: u8,
    pixmap_fd: i32,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_pixmap_from_buffer_checked(
            c.get_raw_conn(),
            pixmap as xcb_pixmap_t,
            drawable as xcb_drawable_t,
            size as u32,
            width as u16,
            height as u16,
            stride as u16,
            depth as u8,
            bpp as u8,
            pixmap_fd as i32,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const BUFFER_FROM_PIXMAP: u8 = 3;

impl base::CookieSeq for xcb_dri3_buffer_from_pixmap_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type BufferFromPixmapCookie<'a> = base::Cookie<'a, xcb_dri3_buffer_from_pixmap_cookie_t>;

impl<'a> BufferFromPixmapCookie<'a> {
    pub fn get_reply(self) -> Result<BufferFromPixmapReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            BufferFromPixmapReply {
                ptr: xcb_dri3_buffer_from_pixmap_reply(
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

pub type BufferFromPixmapReply = base::Reply<xcb_dri3_buffer_from_pixmap_reply_t>;

impl BufferFromPixmapReply {
    pub fn size(&self) -> u32 {
        unsafe { (*self.ptr).size }
    }
    pub fn width(&self) -> u16 {
        unsafe { (*self.ptr).width }
    }
    pub fn height(&self) -> u16 {
        unsafe { (*self.ptr).height }
    }
    pub fn stride(&self) -> u16 {
        unsafe { (*self.ptr).stride }
    }
    pub fn depth(&self) -> u8 {
        unsafe { (*self.ptr).depth }
    }
    pub fn bpp(&self) -> u8 {
        unsafe { (*self.ptr).bpp }
    }
    pub fn pixmap_fds(&self, c: &base::Connection) -> &[i32] {
        unsafe {
            let nfd = (*self.ptr).nfd as usize;
            let ptr = xcb_dri3_buffer_from_pixmap_reply_fds(c.get_raw_conn(), self.ptr);
            std::slice::from_raw_parts(ptr, nfd)
        }
    }
}

pub fn buffer_from_pixmap<'a>(
    c: &'a base::Connection,
    pixmap: xproto::Pixmap,
) -> BufferFromPixmapCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_buffer_from_pixmap(c.get_raw_conn(), pixmap as xcb_pixmap_t);
        BufferFromPixmapCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn buffer_from_pixmap_unchecked<'a>(
    c: &'a base::Connection,
    pixmap: xproto::Pixmap,
) -> BufferFromPixmapCookie<'a> {
    unsafe {
        let cookie =
            xcb_dri3_buffer_from_pixmap_unchecked(c.get_raw_conn(), pixmap as xcb_pixmap_t);
        BufferFromPixmapCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub const FENCE_FROM_FD: u8 = 4;

pub fn fence_from_fd<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    fence: u32,
    initially_triggered: bool,
    fence_fd: i32,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_fence_from_fd(
            c.get_raw_conn(),
            drawable as xcb_drawable_t,
            fence as u32,
            initially_triggered as u8,
            fence_fd as i32,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}

pub fn fence_from_fd_checked<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    fence: u32,
    initially_triggered: bool,
    fence_fd: i32,
) -> base::VoidCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_fence_from_fd_checked(
            c.get_raw_conn(),
            drawable as xcb_drawable_t,
            fence as u32,
            initially_triggered as u8,
            fence_fd as i32,
        );
        base::VoidCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub const FD_FROM_FENCE: u8 = 5;

impl base::CookieSeq for xcb_dri3_fd_from_fence_cookie_t {
    fn sequence(&self) -> c_uint {
        self.sequence
    }
}

pub type FdFromFenceCookie<'a> = base::Cookie<'a, xcb_dri3_fd_from_fence_cookie_t>;

impl<'a> FdFromFenceCookie<'a> {
    pub fn get_reply(self) -> Result<FdFromFenceReply, base::ReplyError> {
        let mut err: *mut xcb_generic_error_t = std::ptr::null_mut();
        let err_ptr = if self.checked {
            &mut err
        } else {
            std::ptr::null_mut()
        };
        let reply = unsafe {
            FdFromFenceReply {
                ptr: xcb_dri3_fd_from_fence_reply(self.conn.get_raw_conn(), self.cookie, err_ptr),
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

pub type FdFromFenceReply = base::Reply<xcb_dri3_fd_from_fence_reply_t>;

impl FdFromFenceReply {
    pub fn fence_fds(&self, c: &base::Connection) -> &[i32] {
        unsafe {
            let nfd = (*self.ptr).nfd as usize;
            let ptr = xcb_dri3_fd_from_fence_reply_fds(c.get_raw_conn(), self.ptr);
            std::slice::from_raw_parts(ptr, nfd)
        }
    }
}

pub fn fd_from_fence<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    fence: u32,
) -> FdFromFenceCookie<'a> {
    unsafe {
        let cookie =
            xcb_dri3_fd_from_fence(c.get_raw_conn(), drawable as xcb_drawable_t, fence as u32);
        FdFromFenceCookie {
            cookie: cookie,
            conn: c,
            checked: true,
        }
    }
}

pub fn fd_from_fence_unchecked<'a>(
    c: &'a base::Connection,
    drawable: xproto::Drawable,
    fence: u32,
) -> FdFromFenceCookie<'a> {
    unsafe {
        let cookie = xcb_dri3_fd_from_fence_unchecked(
            c.get_raw_conn(),
            drawable as xcb_drawable_t,
            fence as u32,
        );
        FdFromFenceCookie {
            cookie: cookie,
            conn: c,
            checked: false,
        }
    }
}
