use std::ffi::{c_char, c_double, c_void};

#[link(name = "tdjson")]
unsafe extern "C" {
    pub fn td_json_client_create() -> *mut c_void;
    pub fn td_json_client_destroy(client: *mut c_void);
    pub fn td_json_client_send(client: *mut c_void, request: *const c_char);
    pub fn td_json_client_receive(client: *mut c_void, timeout: c_double) -> *const c_char;
    pub fn td_json_client_execute(client: *mut c_void, request: *const c_char) -> *mut c_char;
}