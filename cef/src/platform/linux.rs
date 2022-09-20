use crate::types::string::CefString;
use crate::WindowInfo;
use cef_sys::{_cef_rect_t, cef_window_info_t};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;

pub type CefArgs<'a> = &'a [String];

pub(crate) struct CefMainArgsWrapper {
    pub cef: cef_sys::_cef_main_args_t,
    pub keepalive: Vec<CString>,
    pub keepalive2: Vec<*mut c_char>,
}

pub(crate) fn args_to_cef(raw: CefArgs) -> CefMainArgsWrapper {
    // TODO - won't this cause the types to be freed before the pointers?
    let args = raw
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .collect::<Vec<CString>>();
    let mut res = CefMainArgsWrapper {
        cef: cef_sys::_cef_main_args_t {
            argc: 0,
            argv: null_mut(),
        },
        keepalive: args,
        keepalive2: Vec::new(),
    };

    res.keepalive2 = res.keepalive.iter().map(|x| x.as_ptr() as *mut _).collect();
    res.cef.argc = res.keepalive2.len() as i32;
    res.cef.argv = res.keepalive2.as_mut_ptr();

    res
}

pub(crate) fn default_args() -> CefMainArgsWrapper {
    args_to_cef(&std::env::args().collect::<Vec<String>>())
}

pub(crate) type CefCursorInternal = ::std::os::raw::c_ulong;
pub(crate) type CefWindowHandle = ::std::os::raw::c_ulong;

impl<'a> WindowInfo<'a> {
    pub(crate) fn to_cef(&self) -> cef_window_info_t {
        cef_window_info_t {
            window_name: CefString::convert_str_to_cef(self.window_name),
            parent_window: self.parent_window,
            windowless_rendering_enabled: self.windowless_rendering_enabled as i32,
            shared_texture_enabled: self.shared_texture_enabled as i32,
            external_begin_frame_enabled: self.external_begin_frame_enabled as i32,
            window: self.window,
            bounds: _cef_rect_t {
                x: self.x as i32, // @TODO
                y: self.y as i32,
                width: self.width as i32,
                height: self.height as i32,
            },
        }
    }
}
