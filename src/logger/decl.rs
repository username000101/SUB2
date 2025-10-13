use tracing_subscriber;
use parking_lot::FairMutex;
use std::sync::Arc;
use tracing_subscriber::{filter::LevelFilter, reload, prelude::*};
type FilterReloadHandle = reload::Handle<LevelFilter, tracing_subscriber::Registry>;

pub static LOG_FILTER: FairMutex<Option<Arc<FilterReloadHandle>>> = FairMutex::new(None);

pub fn setup_logging() {
    let i_filter =  if cfg!(debug_assertions) { LevelFilter::DEBUG } else { LevelFilter::INFO };
    let (filter, reload_h) = reload::Layer::new(i_filter);
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(tracing_subscriber::fmt::format().with_file(true).with_line_number(true));
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();
    {
        let mut handle_guard = LOG_FILTER.lock();
        *handle_guard = Some(Arc::new(reload_h));
    }
}
