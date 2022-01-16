use x11::xlib::*;
use std::ffi::CString;
use std::ptr;

fn set_status(data: String, display: *mut Display){
    unsafe {
        let c_str = CString::new(data).unwrap();
        XStoreName(display, XDefaultRootWindow(display), c_str.as_ptr() as *const i8);
        XSync(display, 0 as i32);
    }
}

fn main() {
    let handle = unsafe{ XOpenDisplay(ptr::null()) };

    let mut i = 0;
    loop{
        set_status(i.to_string(), handle);
        i+=1;
    }
}
