use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ok {
    
}

use std::ffi::CString;
#[derive(Serialize, Deserialize, Debug)]
pub struct error {
    code: i32,
    message: CString
}