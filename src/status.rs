use std::ptr;
use std::{ffi::CString, os::raw::c_ulong};
use x11::xlib::*;

pub struct Status {
    display: *mut Display,
    window: c_ulong,
}

impl Status {
    pub fn new() -> Self {
        // Setup connection to display
        let dis = unsafe { XOpenDisplay(ptr::null()) };
        let win = unsafe { XDefaultRootWindow(dis) };

        Self {
            display: dis,
            window: win,
        }
    }

    pub fn set_status(&self, data: String) {
        // Create a C char pointer array from data and update the status
        let c_str = CString::new(data.as_str()).unwrap();
        let str_ptr = c_str.as_ptr() as *const i8;

        unsafe {
            XStoreName(self.display, self.window, str_ptr);
            XSync(self.display, 0);
        }
    }
}
