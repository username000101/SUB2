use serde::{Serialize, Deserialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ok {
    
}

use std::ffi::CString;
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug)]
pub struct error {
    code: i32,
    message: CString
}