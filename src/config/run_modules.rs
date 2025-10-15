use crate::config::decl::CONFIGURATION;
use tracing::debug;
pub fn sub2_config_run_modules() {
    let mut lock = CONFIGURATION.write();
    if lock.modules.is_none() {
        return;
    }
    
    let mut loc_procs: Vec<std::process::Child> = vec![];
    {
        let modules = lock.modules.as_ref().unwrap();
        for module in modules {
            debug!("Running module {} aka {}", module.id, module.file.to_str().unwrap());
            let child: std::process::Child;
            if module.prefix.is_none() {
                child = std::process::Command::new(module.file.to_str().unwrap()).spawn().unwrap();
            } else {
                child = std::process::Command::new(module.prefix.as_ref().unwrap())
                    .arg(module.file.to_str().unwrap())
                    .spawn().unwrap();
            }
            loc_procs.push(child);
        }
    }
    lock.processes = loc_procs;
}