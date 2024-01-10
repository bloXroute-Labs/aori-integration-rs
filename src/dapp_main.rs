mod types;
mod util;
mod submit_intent_flow;
mod submit_take_order_flow;
mod gateway;
use std::{env, string};
use std::error::Error;
// use aori_types::events::{AoriResponse};
// use aori_types::events::AoriEvent;
use tokio::task;
use futures::{SinkExt, TryFutureExt};
use futures::StreamExt;
use sha3::{Digest};
use serde::Serialize;
use serde::Deserialize;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tonic::service::Interceptor;
use crate::submit_intent_flow::subscribe_to_make_orders;
use crate::submit_take_order_flow::{start_read_thread_from_aori, process_intent_solutions};

#[tokio::main]
async fn main() {
    // What does this program do?
    // 1. subscribe to order book updates from aori and send make orders as intents to gateway
    // 2. subscribe to intent solutions from the gateway and send take orders to aori
    dotenv::dotenv().ok();
    let bundler_pk_value = std::env::var("BUNDLER_PK").unwrap();
    let bundler_pk_value_clone = bundler_pk_value.clone();

    let auth_header = std::env::var("AUTH_HEADER").unwrap();
    let aori_endpoint = std::env::var("AORI_API_ENDPOINT").unwrap();
    let aori_feed_endpoint = std::env::var("AORI_FEED_ENDPOINT").unwrap();
    let gateway_url = std::env::var("GATEWAY_URL").unwrap();

    let client =  util::create_grpc_client(auth_header.clone(), gateway_url, false).await;
    let cl2 = client.clone();

    let ws_stream = util::create_ws_client(aori_endpoint).await;
    let (write_stream, read_stream) = ws_stream.split();

    let task1 = task::spawn(async move {
        let _ = subscribe_to_make_orders(cl2,
                                         bundler_pk_value_clone.to_string(), aori_feed_endpoint).await;
    });

    let task2 = tokio::spawn(async move {
        let _ = start_read_thread_from_aori(read_stream).await;
    });

    let task3 = tokio::spawn(async move {
        let _ = process_intent_solutions(client,
                                         bundler_pk_value.to_string(), auth_header, write_stream).await;

    });

    tokio::try_join!(task1, task2, task3).expect("Failed to join tasks");

}
