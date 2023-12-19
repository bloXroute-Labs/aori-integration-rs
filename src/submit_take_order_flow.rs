use std::future::Future;
use std::process::exit;
use std::str::FromStr;
use std::sync::{Arc};
use crate::gateway::gateway_client::GatewayClient;
use tonic::{transport::Channel};
use tonic::codegen::InterceptedService;
use crate::{types, util};
use sha3::{Digest, Keccak256};
use futures::{SinkExt, TryFutureExt};
use futures::stream::{SplitSink, SplitStream};
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::WebSocketStream;
use futures::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::protocol::Message;
use crate::gateway::{IntentSolutionsRequest};


pub async fn process_intent_solutions(
    mut client: GatewayClient<InterceptedService<Channel, types::BlxrCredentials>>,
    bundler_private_key: String,
    auth_header: String,
    write_stream: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>) -> Result<(), Box<dyn std::error::Error>> {

    let secp = secp256k1::Secp256k1::new();
    let sk = secp256k1::SecretKey::from_str(&bundler_private_key).unwrap();
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

    let dapp_address = util::public_key_to_address(pk);
    let hash = Keccak256::digest(dapp_address.clone());

    let msg = secp256k1::Message::from_slice(&hash).unwrap();
    let intent_sig = secp.sign_ecdsa_recoverable(&msg, &sk);

    let mut signature_with_recovery: Vec<u8> = vec![];
    let signature_and_recovery = intent_sig.serialize_compact();
    // Append the recovery id (1 byte) to the serialized signature
    signature_with_recovery.append(&mut signature_and_recovery.1.to_vec());
    signature_with_recovery.push(signature_and_recovery.0.to_i32() as u8);

    let message = IntentSolutionsRequest {
        dapp_address: dapp_address.clone(),
        hash: hash.to_vec(),
        auth_header: auth_header,
        signature: signature_with_recovery,
    };

    let response = client.intent_solutions(message).await?;
    let mut streaming_body = response.into_inner();

    let write_stream_arc = Arc::new(Mutex::new(write_stream));
    while let Some(value) = streaming_body.message().await? {
        let res = send_take_orders_aori(write_stream_arc.clone(), value.intent_solution);
        match res.await {
            Ok(_) => {
                println!("sent take order to aori complete");
            }
            Err(e) => {
                eprintln!("sending take order to aori has error: {:?}", e);
            }
        }
            // .await.expect("failed to send take order to aori");
        // match serde_json::from_slice::<>(&) {
        //     Ok(solution) => {
        //
        //     },
        //     Err(e) => {
        //         eprintln!("JSON deserialization error: {:?}", e);
        //     }
        // }
        ()
    }
    exit(1);
    Ok(())
}

pub async fn start_read_thread_from_aori(mut read_stream: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Result<(), Box<dyn std::error::Error>> {
    println!("start_read_thread_from_aori1");
    // let mut ws_stream = read_stream().unwrap();

    while let Some(response) = read_stream.next().await {
        match response {
            Ok(msg) => {
                println!("response from_aori : {}", msg);
            }
            Err(e) => {
                eprintln!("start_read_thread_from_aori : Error receiving response: {:?}", e);
            }
        }
    }
    println!("end of start_read_thread_from_aori");
    Ok(())
}

pub async fn send_take_orders_aori(
    mut write_stream: Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>,
    take_order_request: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let request_message = Message::Binary(take_order_request);
    let res = write_stream.lock().await.send(request_message).await;
    match res {
        Ok(_) => {
            println!("wrote take order to aori");
        }
        Err(e) => { eprintln!("send_take_orders_aori : Error writing to aori response: {:?}", e);}
    }
    Ok(())
}
