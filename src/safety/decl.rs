use std::panic;
use crate::config::decl::CONFIGURATION;
use tracing::warn;
static EXIT_FN: fn() = || {
    let mut cfg = CONFIGURATION.write();
    let procs = &mut cfg.processes;
    for proc in procs {
        if proc.try_wait().unwrap().is_none() {
            warn!("Killing process with id {}", proc.id());
            proc.kill().unwrap();
        }
    }
    std::process::exit(0);
};

pub fn sub2_set_handlers() {
    ctrlc::set_handler(EXIT_FN).unwrap();
    panic::set_hook(Box::new(|info| {
        warn!("Called panic! by: {}", info.to_string());
        EXIT_FN();
    }));
}