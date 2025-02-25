use async_tungstenite::tokio::accept_async;
use tokio::net::TcpListener;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::domain::blockchain::Blockchain;

pub async fn start_p2p_server(blockchain: Arc<Mutex<Blockchain>>) {
    let listener = TcpListener::bind("go env").await.unwrap();;

    while let Ok((stream, _)) = listener.accept().await {
        let blockchain = blockchain.clone();
        tokio::spawn(async move {
            let ws_stream = accept_async(stream).await.unwrap();
            println!("New peer connected!");
        });
    }
}
