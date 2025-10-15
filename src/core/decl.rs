use crate::{td, sub2_get_version};
use tracing::info;
use crate::auth;
use crate::config;
pub fn sub2_start_main_loop() {
    info!("Starting SUB2 v{}", sub2_get_version!()());

    {
        let mut lock = td::interface::CLIENT.lock();
        lock.execute(td::requests::setLogVerbosityLevel(1));
        lock.start_updates_loop();
    }
    
    auth::decl::sub2_tdlib_auth();
    config::interface::run_modules();

    info!("Nothing to do");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}