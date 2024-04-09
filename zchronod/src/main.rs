mod zchronod;

use std::path::PathBuf;
use zchronod_tools::tokio_zhronod;
use zchronod_node_api::config;
use structopt::StructOpt;
use zchronod_node_api::error::{ZchronodConfigError, ZchronodConfigResult};
use tracing::*;
use zchronod_node_api::config::ZchronodConfig;

#[derive(StructOpt)]
struct ZchronodCli {
    #[structopt(short = "c", long = "config", parse(from_os_str), help = "Yaml file only")]
    config_path: std::path::PathBuf,
}

fn main() {
    println!("start zchronod");
    tokio_zhronod::block_forever_on(async_main());
}

async fn async_main() {
    let args = ZchronodCli::from_args();
    let zchronod_config = construct_node_config(args.config_path.clone());
    let db_root_path = zchronod_config.storage_root_path.unwrap();

    //todo metrics init

    let zchronod = build_zchronod(zchronod_config).await;

    // shutdown zchronod
    tokio::signal::ctrl_c()
        .await
        .unwrap_or_else(|e| tracing::error!("Could not handle termination signal: {:?}", e));
    tracing::info!("Gracefully shutting down conductor...");
    let shutdown_result = zchronod.shutdown().await;
   // handle_shutdown(shutdown_result);
}

async fn build_zchronod(config: ZchronodConfig) {
    todo!()
}

fn construct_node_config(config_path: PathBuf) -> config::ZchronodConfig {
    match config::ZchronodConfig::load_config(config_path) {
        Err(ZchronodConfigError::ConfigMissing(_)) => {
            std::process::exit(ERROR_CODE);
        }
        Err(ZchronodConfigError::SerializationError(err)) => {
            std::process::exit(ERROR_CODE);
        }
        result => {
            result.expect("failed to load zhronod config")
        }
    }
}

/// start zchronod node error code for loading config
pub const ERROR_CODE: i32 = 42;