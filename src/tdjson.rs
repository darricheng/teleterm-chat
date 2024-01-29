use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

// Definitions courtesy of rust-tdlib
// https://github.com/antonio-antuan/rust-tdlib/blob/master/src/tdjson.rs
pub type ClientId = i32;

extern "C" {
    fn td_create_client_id() -> c_int;
    fn td_send(client_id: c_int, request: *const c_char);
    fn td_receive(timeout: c_double) -> *const c_char;
    fn td_execute(request: *const c_char) -> *const c_char;
}

pub fn new_client() -> ClientId {
    unsafe { td_create_client_id() }
}

pub fn send(client_id: ClientId, request: &str) {
    let cstring = CString::new(request).unwrap();
    unsafe { td_send(client_id, cstring.as_ptr()) }
}

pub fn execute(request: &str) -> Option<String> {
    let cstring = CString::new(request).unwrap();
    let result = unsafe {
        td_execute(cstring.as_ptr())
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    };
    result
}

pub fn receive(timeout: f64) -> Option<String> {
    unsafe {
        td_receive(timeout)
            .as_ref()
            .map(|response| CStr::from_ptr(response).to_string_lossy().into_owned())
    }
}
