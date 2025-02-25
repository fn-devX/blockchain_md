mod config;
mod domain {
    pub mod block;
    pub mod transaction;
    pub mod blockchain;
    pub mod crypto;
}
mod infrastructure {
    pub mod network;
    pub mod database;
}
mod application {
    pub mod api;
    pub mod services;
}
mod utils {
    pub mod logger;
}

use config::Config;
use std::sync::Arc;
use tokio::sync::Mutex;
use domain::blockchain::Blockchain;
use warp::Filter;
use tokio::task;

#[tokio::main]
async fn main() {
    env_logger::init();

    let config = Config::load();
    let blockchain = Arc::new(Mutex::new(Blockchain::new(config.blockchain.difficulty)));

    let api_routes = application::api::create_routes(blockchain.clone());
    let network_task = task::spawn(infrastructure::network::start_p2p_server(blockchain.clone()));

    warp::serve(api_routes).run((config.network.host.parse().unwrap(), config.network.port)).await;
    network_task.await.unwrap();
}
