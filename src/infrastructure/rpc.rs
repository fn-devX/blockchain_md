use warp::Filter;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::blockchain::Blockchain;

pub fn json_rpc_routes(blockchain: Arc<Mutex<Blockchain>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let blockchain_filter = warp::any().map(move || blockchain.clone());

    warp::path("rpc")
        .and(warp::post())
        .and(warp::body::json())
        .and(blockchain_filter)
        .map(|body: serde_json::Value, blockchain: Arc<Mutex<Blockchain>>| {
            let method = body["method"].as_str().unwrap_or("");
            let params = body["params"].as_array().unwrap_or(&vec![]);

            let response = match method {
                "get_chain" => {
                    let blockchain = blockchain.blocking_lock();
                    json!({ "result": blockchain.chain })
                }
                _ => json!({ "error": "Unknown method" }),
            };

            warp::reply::json(&response)
        })
}
