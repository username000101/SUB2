pub mod auth;
pub mod config;
pub mod core;

pub mod cli;
pub mod td;
pub mod logger;
pub mod server;
pub mod safety;

#[tokio::main]
async fn main() {
    let sub2_uptime_start = std::time::Instant::now();
    safety::interface::set_handlers();
    logger::log::setup_logging();
    tokio::spawn(server::rpc::interface::start_rpc_server());
    cli::interface::parse_args();
    core::interface::start_main_loop();
    tracing::info!("Uptime: {}", sub2_uptime_start.elapsed().as_millis());
}