use prometheus::{IntCounter, Encoder, TextEncoder};
use lazy_static::lazy_static;
use warp::Filter;

lazy_static! {
    static ref BLOCK_COUNT: IntCounter = IntCounter::new("block_count", "Number of blocks").unwrap();
}

pub fn monitoring_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("metrics").map(|| {
        let encoder = TextEncoder::new();
        let mut buffer = vec![];
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap();
        warp::reply::with_header(buffer, "Content-Type", "text/plain; version=0.0.4")
    })
}
