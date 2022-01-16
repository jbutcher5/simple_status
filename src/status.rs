use std::ffi::CString;
use std::ptr;
use x11::xlib::*;

pub struct Status {
    pub data: String,
    display: *mut Display,
}

impl Status {
    pub fn new(data: String) -> Self {
        Self {
            data,
            display: unsafe { XOpenDisplay(ptr::null()) },
        }
    }

    pub fn set_status(&self) {
        unsafe {
            let c_str = CString::new(self.data.as_str()).unwrap();
            XStoreName(
                self.display,
                XDefaultRootWindow(self.display),
                c_str.as_ptr() as *const i8,
            );
            XSync(self.display, 0_i32);
        }
    }
}
