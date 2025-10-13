#![allow(non_snake_case)]

use tracing::{info, debug, warn};
use serde_json::{json, Value};
use std::path::PathBuf;
use crate::sub2_get_version;
pub fn setLogVerbosityLevel(new_verbosity_level: u32) -> Value {
    json!(
        {
            "@type": "setLogVerbosityLevel",
            "new_verbosity_level": new_verbosity_level,
        }
    )
}

pub fn close() -> Value {
    json!(
        {
            "@type": "close"
        }
    )
}

pub fn setTdlibParameters() -> Value {
    use crate::config::interface;
    let config = interface::get_config().unwrap();
    let database_dir = PathBuf::from(&config.read().directory).join("tdlib").join("database");
    let files_dir = PathBuf::from(&config.read().directory).join("tdlib").join("files");
    let api_id = config.read().api_id;
    let api_hash = config.read().api_hash.clone();
    let device_model = if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else if cfg!(target_os = "macos") {
        "MacOS".to_string()
    } else if cfg!(target_os = "android") {
        "Android".to_string()
    } else {
        "Unknown".to_string()
    };

    let application_version = sub2_get_version!()();
    let locale = match sys_locale::get_locale() {
        Some(loc) => loc.to_string(),
        _ => {
            warn!("sys_locale::get_locale() returned None, using default 'en-EN' locale");
            "en-EN".to_string()
        }
    };

    json!(
        {
            "@type": "setTdlibParameters",
            "database_directory": json!(database_dir.to_str().unwrap()),
            "files_directory": json!(files_dir.to_str().unwrap()),
            "use_file_database": json!(true),
            "use_message_database": json!(true),
            "use_secret_chats": json!(true),
            "use_chat_info_database": json!(true),
            "api_id": json!(api_id),
            "api_hash": json!(api_hash),
            "system_language_code": json!(locale),
            "application_version": json!(application_version),
            "device_model": json!(device_model),

        }
    )
}

pub fn getAuthorizationState() -> Value {
    json!(
        {
            "@type": "getAuthorizationState",
        }
    )
}

pub fn setAuthenticationPhoneNumber(phone_number: String) -> Value{
    json!(
        {
            "@type": "setAuthenticationPhoneNumber",
            "phone_number": json!(phone_number),
        }
    )
}

pub fn checkAuthenticationCode(code: String) -> Value {
    json!(
        {
            "@type": "checkAuthenticationCode",
            "code": json!(code),
        }
    )
}

pub fn checkAuthenticationPassword(password: String) -> Value {
    json!(
        {
            "@type": "checkAuthenticationPassword",
            "password": json!(password),
        }
    )
}

pub fn getMe() -> Value {
    json!(
        {
            "@type": "getMe",
        }
    )
}