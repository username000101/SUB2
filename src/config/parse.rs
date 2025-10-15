use std::path::PathBuf;
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use serde_json::from_str;
use crate::config::decl;
use tracing::{debug, warn, info};

pub fn parse_config(config: PathBuf) {
    if !config.exists() {
        panic!("Failed to open file: file {} does not exist", config.display());
    }
    
    let cfg: &Lazy<RwLock<decl::Configuration>> = &decl::CONFIGURATION;
    let cfg_str = std::fs::read_to_string(config).unwrap();
    let parse_result = from_str::<decl::Configuration>(cfg_str.as_str()).unwrap();
    
    if !parse_result.directory.exists() {
        panic!("Failed to parse file: directory {} does not exist", parse_result.directory.display());
    }
    
    debug!("Writing these values in the configuration: \n{:#?}", parse_result);
    
    cfg.write().directory = parse_result.directory;
    cfg.write().api_id = parse_result.api_id;
    cfg.write().api_hash = parse_result.api_hash;
    cfg.write().ready = true;
    if !parse_result.blocked_requests.is_none() {
        let breqs = parse_result.blocked_requests.unwrap();
        info!("Running with these blocked requests:\n{:#?}", &breqs);
        cfg.write().blocked_requests = Some(breqs);
    }
    cfg.write().panic_if_module_starting_error = if parse_result.panic_if_module_starting_error.is_none() {
        cfg.read().panic_if_module_starting_error
    } else {
        parse_result.panic_if_module_starting_error
    };
    cfg.write().version = parse_result.version;
    if parse_result.modules.is_none() {
        warn!("Not found section 'modules'");
    } else {
        cfg.write().modules = parse_result.modules;
    }
}