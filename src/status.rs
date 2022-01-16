use x11::xlib::*;
use std::ffi::CString;
use std::ptr;

pub struct Status {
    data: &'static str,
    display: *mut Display
}

impl Status {
    pub fn new(data: &'static str) -> Self {
        Self {
            data,
            display: unsafe{ XOpenDisplay(ptr::null()) }
        }
    }

    pub fn set_status(&self){
        unsafe {
            let c_str = CString::new(self.data).unwrap();
            XStoreName(self.display, XDefaultRootWindow(self.display), c_str.as_ptr() as *const i8);
            XSync(self.display, 0 as i32);
        }
    }
}
