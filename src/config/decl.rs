use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    pub file: PathBuf,
    pub id: String,
    pub prefix: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    #[serde(skip)]
    pub ready: bool,
    pub api_id: i32,
    pub api_hash: String,
    pub version: String,
    pub panic_if_module_starting_error: Option<bool>,
    #[serde(skip)]
    pub spoofed_version: Option<String>,
    #[serde(alias = "dir")]
    pub directory: PathBuf,
    pub modules: Option<Vec<Module>>,
    #[serde(skip)]
    pub process: Vec<std::process::Child>,
    pub blocked_requests: Option<Vec<String>>,
}

use parking_lot::RwLock;
use once_cell::sync::Lazy;
pub static CONFIGURATION: Lazy<RwLock<Configuration>> = Lazy::new(|| {
    RwLock::new(
        Configuration {
            ready: false,
            api_id: 0,
            api_hash: "".to_string(),
            version: "".to_string(),
            spoofed_version: None,
            directory: PathBuf::new(),
            modules: None,
            panic_if_module_starting_error: Some(true),
            process: Vec::new(),
            blocked_requests: None,
        }
    )
});

#[macro_export] macro_rules! sub2_get_version {
    () => {
        || -> String {
            use crate::config::decl::CONFIGURATION;
            match &CONFIGURATION.read().spoofed_version {
                None => CONFIGURATION.read().version.clone(),
                Some(s_ver) => s_ver.clone()
            }
        }
    };
}