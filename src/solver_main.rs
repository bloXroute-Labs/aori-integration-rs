mod types;
mod gateway;
mod util;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::env;
use num_bigint::BigInt;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use aori_types::events::OrderParameters;
use futures::SinkExt;
use rand::Rng;
use sha3::{Digest, Keccak256};
use tonic::codegen::InterceptedService;
use aori_requests::requests::{create_take_order_payload};
use tonic::{Response, Status, transport::Channel};
use tonic::service::Interceptor;
use crate::gateway::gateway_client::GatewayClient;
use crate::types::{AoriMakeOrder, AoriMakeOrderIntent, AoriTakeOrderIntent, BlxrCredentials};
use num_traits::cast::FromPrimitive;
use serde_json::to_string;
use eyre::Context;

use crate::gateway::{SubmitIntentRequest, SubmitIntentSolutionReply, SubmitIntentSolutionRequest};

#[tokio::main]

async fn main() {
    dotenv::dotenv().ok();
    let solver_pk = std::env::var("SOLVER_PK").unwrap();
    let solver_public_key = std::env::var("SOLVER_PUBLIC_KEY").unwrap();
    let auth_header = std::env::var("AUTH_HEADER").unwrap();
    let gateway_url = std::env::var("GATEWAY_URL").unwrap();
    let node_url = std::env::var("NODE_URL").unwrap();
    env::set_var("PRIVATE_KEY", solver_pk.clone());
    env::set_var("WALLET_ADDRESS", solver_public_key);
    env::set_var("NODE_URL", node_url);
    let client =  util::create_grpc_client(
        auth_header.clone(), gateway_url
    ).await;

    let _ = subscribe_to_gateway_intents(client, solver_pk, auth_header).await;
}

pub async fn subscribe_to_gateway_intents(mut client: GatewayClient<InterceptedService<Channel, types::BlxrCredentials>>,
                                          solver_private_key: String,
                                          auth_header: String) -> Result<(), Box<dyn std::error::Error>> {
    let secp = secp256k1::Secp256k1::new();
    let sk = secp256k1::SecretKey::from_str(&solver_private_key).unwrap();
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

    let solver_address = util::public_key_to_address(pk);
    let hash = Keccak256::digest(solver_address.clone());

    let msg = secp256k1::Message::from_slice(&hash).unwrap();
    let intent_sig = secp.sign_ecdsa_recoverable(&msg, &sk);

    let mut signature_with_recovery: Vec<u8> = vec![];
    let signature_and_recovery = intent_sig.serialize_compact();
    // Append the recovery id (1 byte) to the serialized signature
    signature_with_recovery.append(&mut signature_and_recovery.1.to_vec());
    signature_with_recovery.push(signature_and_recovery.0.to_i32() as u8);
    let message = gateway::IntentsRequest {
        auth_header: auth_header.clone(),
        solver_address: solver_address.clone(),
        hash: hash.to_vec(),
        signature: signature_with_recovery,
        filters: "".to_string(),
        from_timestamp: None,
    };

    let response = client.intents(message).await?;
    let mut streaming_body = response.into_inner();


    while let Some(value) = streaming_body.message().await? {
        match serde_json::from_slice::<types::AoriMakeOrder>(&value.intent) {
            Ok(intent) => {
                let intent = AoriMakeOrderIntent {
                    intent_id: value.intent_id,
                    make_order: intent,
                };

                send_take_orders_to_gateway(client.clone(), solver_private_key.clone(), auth_header.clone(), intent).await;
            },
            Err(e) => {
                eprintln!("subscribe_to_gateway_intents : JSON deserialization error: {:?}", e);
            }
        }
    }
    Ok(())
}


async fn send_take_orders_to_gateway(
    mut client: GatewayClient<InterceptedService<Channel, BlxrCredentials>>,
    solver_private_key: String,
    auth_header: String,
    intent: AoriMakeOrderIntent) {
    let intent_id = intent.intent_id.clone();
    let make_order = intent.make_order;


    let mut provider = util::new_from_env()
        .await
        .expect("Failed to create API provider.");

    dotenv::dotenv().ok();

    let wallet = &provider.wallet_addr.clone().unwrap();
    let sell_token = make_order.result.data.order.parameters.offer[0].token.clone();
    let buy_token = make_order.result.data.order.parameters.consideration[0].token.clone();
    let sell_amount = make_order.result.data.order.parameters.offer[0].start_amount.clone();
    let buy_amount = make_order.result.data.order.parameters.offer[0].end_amount.clone();

    let mut order_params =
        OrderParameters::limit_order(wallet, &sell_token, &sell_amount, &buy_token, &buy_amount)
            .to_order_components();

    let order_id = &make_order.result.data.order_hash.clone();
    let seat_id = 0;
    let chain_id = make_order.result.data.chain_id as u64;

    let wallet = provider.wallet.as_ref().ok_or(eyre::eyre!(
            "wallet: add wallet private key to auth wallet."
        ));
    let take_order_payload = create_take_order_payload(
        &provider.last_id,
        wallet.unwrap(),
        &mut order_params,
        order_id,
        &seat_id,
        &chain_id,
    );
    let solution_bytes = to_string(&take_order_payload.unwrap()).expect("Serialization failed");

    let secp = secp256k1::Secp256k1::new();
    let sk = secp256k1::SecretKey::from_str(&solver_private_key).unwrap();
    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);

    let solution_hash = Keccak256::digest(solution_bytes.clone());
    let msg = secp256k1::Message::from_slice(&solution_hash).unwrap();
    let intent_sig = secp.sign_ecdsa_recoverable(&msg, &sk);

    let mut signature_with_recovery: Vec<u8> = vec![];
    let signature_and_recovery = intent_sig.serialize_compact();
    // Append the recovery id (1 byte) to the serialized signature
    signature_with_recovery.append(&mut signature_and_recovery.1.to_vec());
    signature_with_recovery.push(signature_and_recovery.0.to_i32() as u8);

    let message = SubmitIntentSolutionRequest {
        auth_header: auth_header,
        solver_address:  util::public_key_to_address(pk),
        intent_id: intent_id.to_string(),
        intent_solution: solution_bytes.into_bytes(),
        hash: solution_hash.to_vec(),
        signature: signature_with_recovery,
    };

    let res = client.submit_intent_solution(message).await;
    match res {
        Ok(_) => {
            eprintln!("submitted intent solution");
        }
        Err(e) => {
            eprintln!("error when submitting intent solution : {:?}", e);
        }
    }
    ()
}


fn add_fee(str: &str) -> String {
    let num = f64::from_str(str).expect("Failed to parse string as float");
    let num_with_fee = num * 1.0003;

    let rounded_num = BigInt::from_f64(num_with_fee).expect("Failed to convert float to BigInt");

    rounded_num.to_string()
}

fn convert_make_order_intent(solver_address: String, mut make_order_intent: AoriMakeOrder) -> AoriTakeOrderIntent {
    let offers = &make_order_intent.result.data.order.parameters.offer;
    make_order_intent.result.data.order.parameters.offerer = solver_address.clone();

    let new_considerations: Vec<types::ConsiderationDetail> = offers
        .iter()
        .map(|offer| types::ConsiderationDetail {
            item_type: offer.item_type,
            token: offer.token.clone(),
            identifier_or_criteria: offer.identifier_or_criteria.to_string(),
            start_amount: offer.start_amount.to_string(),
            recipient: solver_address.clone(),
            end_amount: offer.end_amount.to_string(),
        })
        .collect();

    make_order_intent.result.data.order.parameters.consideration = new_considerations;

    let considerations = &make_order_intent.result.data.order.parameters.consideration;
    let new_offers: Vec<types::OfferDetail> = considerations
        .iter()
        .map(|consideration| types::OfferDetail {
            item_type: consideration.item_type,
            token: consideration.token.clone(),
            identifier_or_criteria: consideration.identifier_or_criteria.clone(),
            start_amount: add_fee(&consideration.start_amount),
            end_amount: add_fee(&consideration.end_amount),
        })
        .collect();

    make_order_intent.result.data.order.parameters.offer = new_offers;

    let take_order = types::TakeOrderRequest {
        id: rand::thread_rng().gen_range(0..1000000),
        jsonrpc: "2.0".to_string(),
        method: "aori_takeOrder".to_string(),
        params: vec![types::TakeOrderRequestParams {
            order_id: make_order_intent.result.data.order_hash.clone(),
            order: make_order_intent.result.data.order.clone(),
            seat_id: 0,
            chain_id: make_order_intent.result.data.chain_id,
        }],
    };

    AoriTakeOrderIntent {
        intent_id: make_order_intent.id.unwrap().to_string(),
        take_order_request: take_order,
    }
}