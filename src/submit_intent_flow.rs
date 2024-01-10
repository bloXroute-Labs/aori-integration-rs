use std::str::FromStr;
use std::sync::{Arc, Mutex};
use serde_json::to_vec;
use crate::gateway::gateway_client::GatewayClient;
use tonic::{transport::Channel, Status, Request, Response};
use tonic::codegen::InterceptedService;
use crate::{types, util};
use aori_requests::requests::create_subscribe_orderbook_payload;
use crate::gateway::{SubmitIntentReply, SubmitIntentRequest, SubmitIntentRequestData};
use sha3::{Digest, Keccak256};
use futures::SinkExt;
use futures::StreamExt;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn subscribe_to_make_orders(mut client: GatewayClient<InterceptedService<Channel, types::BlxrCredentials>>, bundler_private_key: String, aori_feed_endpoint: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("subscribing to {} for make order updates", aori_feed_endpoint);
    let (mut ws_stream, _) = connect_async(aori_feed_endpoint)
        .await
        .expect("Error connecting to the URL");

    let id: Arc<Mutex<u64>> = Arc::new(Mutex::new(1u64));

    let request_json = create_subscribe_orderbook_payload(&id);
    let request_text = request_json.to_string();
    let request_message = Message::Text(request_text);

    ws_stream.send(request_message).await?;

    while let Some(response) = ws_stream.next().await {
        match response {
            Ok(msg) => {
                if msg.is_text() {
                    if let Ok(text) = msg.to_text() {
                        println!("events received from aori");
                        match serde_json::from_str::<types::AoriResponse>(text) {
                            Ok(view_orderbook_response) => {
                                // Successfully deserialized, now you can use view_orderbook_response
                                // println!("Deserialized response: {:?}", view_orderbook_response);
                                //
                                // let order = view_orderbook_response;
                                // println!("It's a orderCreated event with data: {}", order.order_hash);
                                // let res = submit_intent(bundler_private_key.clone(), Box::new(order)).await;
                                // match res {
                                //     Ok(_) => {}
                                //     Err(_) => {}
                                // }

                                match view_orderbook_response.result.result_type.as_str() {
                                    "OrderCreated" => {
                                        println!("It's a orderCreated event with data: {}", view_orderbook_response.result.data.order_hash);
                                        let res = submit_intent(client.clone(), bundler_private_key.clone(), Box::new(view_orderbook_response));
                                        match res.await {
                                            Ok(_) => {
                                                println!("submitted intent");
                                            }
                                            Err(e) => {
                                                eprintln!("submit error: {:?}", e);
                                            }
                                        }

                                    }
                                    _ => {
                                        // println!("It's not a Subscribed event");
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("subscribe_to_make_orders : JSON deserialization error: {:?}", e);
                            }
                        }
                    }
                } else if msg.is_binary() {
                    println!("Received a binary message");
                }
            }
            Err(e) => {
                eprintln!("Error receiving response: {:?}", e);
            }
        }
    }

    Ok(())
}

pub async fn submit_intent(mut client: GatewayClient<InterceptedService<Channel, types::BlxrCredentials>>, bundler_private_key: String, order: Box<types::AoriResponse>) -> Result<(), Box<dyn std::error::Error>> {

    let secp = secp256k1::Secp256k1::new();
    let sk = secp256k1::SecretKey::from_str(&bundler_private_key).unwrap();
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

    let json_data = to_vec(&order).expect("Failed to serialize to JSON");

    let intent_hash = Keccak256::digest(json_data.clone());
    let msg = secp256k1::Message::from_slice(&intent_hash).unwrap();
    let intent_sig = secp.sign_ecdsa_recoverable(&msg, &sk);

    let mut signature_with_recovery: Vec<u8> = vec![];
    let signature_and_recovery = intent_sig.serialize_compact();
    // Append the recovery id (1 byte) to the serialized signature
    signature_with_recovery.append(&mut signature_and_recovery.1.to_vec());
    signature_with_recovery.push(signature_and_recovery.0.to_i32() as u8);

    let message = SubmitIntentRequest {
        sender_address: util::public_key_to_address(pk),
        data: Some(SubmitIntentRequestData{
            dapp_address:  util::public_key_to_address(pk),
            nonce: "1".to_string(),
            intent: json_data,
            expiry_duration: None,
        }),
        hash: intent_hash.to_vec(),
        signature: signature_with_recovery,
    };
    let res = client.submit_intent(message).await;
    match res {
        Ok(_) => {
            println!("Submission successful");
        }
        Err(status) => {
            println!("Submission failed: {:?}", status);
        }
    }
    Ok(())
}
