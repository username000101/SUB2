use crate::logger::decl;

use tracing::info;
use tracing::metadata::LevelFilter;
use crate::logger::decl::LOG_FILTER;

pub fn setup_logging() {
    decl::setup_logging();
    info!("Logger setup successfully");
}

pub fn disable_logging_for<F>(mut f: F) where F: FnMut() {
    let arc_h = {
        let lock = LOG_FILTER.lock();
        lock.as_ref().cloned().unwrap()
    };

    let current_log_filter = arc_h.with_current(|filter| *filter).unwrap();
    arc_h.modify(|filter| *filter = LevelFilter::OFF).unwrap();
    f();
    arc_h.modify(|filter| *filter = current_log_filter).unwrap()
}