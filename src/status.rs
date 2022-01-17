use std::ptr;
use std::{ffi::CString, os::raw::c_ulong};
use x11::xlib::*;

pub struct Status {
    pub data: String,
    display: *mut Display,
    window: c_ulong,
}

impl Status {
    pub fn new(data: String) -> Self {
        let dis = unsafe { XOpenDisplay(ptr::null()) };
        let win = unsafe { XDefaultRootWindow(dis) };

        Self {
            data,
            display: dis,
            window: win,
        }
    }

    pub fn set_status(&self) {
        let c_str = CString::new(self.data.as_str()).unwrap();
        let str_ptr = c_str.as_ptr() as *const i8;

        unsafe {
            XStoreName(self.display, self.window, str_ptr);
            XSync(self.display, 0);
        }
    }
}
