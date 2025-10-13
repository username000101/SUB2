use clap::Parser;
use std::path::PathBuf;
use crate::config;
use config::decl;
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    // Configuration file
    #[arg(short, long)]
    config: PathBuf,

    // Spoof the version
    #[clap(short, long)]
    spoof_version: Option<String>,
}

use tracing::{info, debug};
pub fn parse_cli_args() {
    let args = Args::parse();

    if !args.config.exists() {
        panic!("Not found config file: {}", args.config.display());
    } else {
        config::interface::parse(args.config.clone());
    }
    
    if let Some(spoofed_ver) = args.spoof_version {
        info!("Spoofing SUB version to: {}", spoofed_ver);
        decl::CONFIGURATION.write().spoofed_version = Some(spoofed_ver);
    }
}