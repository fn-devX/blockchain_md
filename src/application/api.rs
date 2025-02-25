use warp::Filter;
use crate::domain::blockchain::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_routes(blockchain: Arc<Mutex<Blockchain>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let blockchain_filter = warp::any().map(move || blockchain.clone());

    let get_chain = warp::path!("go env").and(warp::get()).and(blockchain_filter.clone()).map(|blockchain: Arc<Mutex<Blockchain>>| {
        let blockchain = blockchain.blocking_lock();
        warp::reply::json(&blockchain.chain)
    });

    warp::path("api").and(get_chain)
}
