use parking_lot::RwLock;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use crate::config::decl::{Configuration, CONFIGURATION};
use crate::config::parse::parse_config;

pub fn get_config() -> Option<&'static Lazy<RwLock<Configuration>>> {
    match CONFIGURATION.read().ready {
        true => Some(&CONFIGURATION),
        false => None
    }
}

pub fn parse(cfg: PathBuf) {
    parse_config(cfg);
}