use std::sync::{Arc, Mutex};
use sha3::{Digest, Keccak256};
use alloy_primitives::hex;
use aori_requests::aori_provider::AoriProvider;
use eyre::Context;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use gateway::gateway_client::GatewayClient;
use crate::{gateway, types};
use tokio_tungstenite::connect_async;

use ethers::{
    prelude::{LocalWallet, Ws},
    providers::{Middleware, Provider},
    signers::Signer,
    types::Signature,
};

async fn connect_websockets() -> anyhow::Result<(
    WebSocketStream<MaybeTlsStream<TcpStream>>,
    WebSocketStream<MaybeTlsStream<TcpStream>>,
)> {
    let REQUEST_URL = "wss://staging.api.aori.io";
    let MARKET_FEED_URL = "wss://staging.feed.aori.io";
    let ((request_conn, _), (feed_conn, _)) =
        tokio::try_join!(connect_async(REQUEST_URL), connect_async(MARKET_FEED_URL))
            .expect("Failed to connect to websockets");

    Ok((request_conn, feed_conn))
}
async fn initialize_wallet(
    key: &str,
    address: &str,
    node: String,
) -> anyhow::Result<(LocalWallet, u64, String, String)> {
    let pv = Provider::<Ws>::connect(&node).await?;
    let chain_id = pv.get_chainid().await?.low_u64();
    let wallet = key.parse::<LocalWallet>()?.with_chain_id(chain_id);
    let sig: Signature = wallet.sign_message(address).await?;
    let wallet_addr = address.to_string();
    let wallet_sig = format!("0x{}", sig).to_string();

    Ok((wallet, chain_id, wallet_addr, wallet_sig))
}

pub async fn new_from_env() -> anyhow::Result<AoriProvider, anyhow::Error> {
    let key = std::env::var("PRIVATE_KEY")
        .context("missing PRIVATE_KEY")
        .unwrap();
    let address = std::env::var("WALLET_ADDRESS")
        .context("missing WALLET_ADDRESS")
        .unwrap();
    let node = std::env::var("NODE_URL")
        .context("missing NODE_URL")
        .unwrap();

    let (request_conn, feed_conn) = connect_websockets().await?;
    let (wallet, chain_id, wallet_addr, wallet_sig) =
        initialize_wallet(&key, &address, node).await?;

    Ok(AoriProvider {
        request_conn,
        feed_conn,
        wallet: Some(wallet),
        chain_id: Some(chain_id),
        last_id: Arc::new(Mutex::new(0)),
        wallet_addr: Some(wallet_addr),
        wallet_sig: Some(wallet_sig),
    })
}
pub async fn create_ws_client(aori_endpoint: String) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    println!("subscribing to {} for send take orders", aori_endpoint);
    let (mut ws_stream, _) = connect_async(aori_endpoint)
        .await
        .expect("Error connecting to the URL");
    ws_stream
}
pub async fn create_grpc_client(auth_header: String, gateway_url: String) -> GatewayClient<InterceptedService<Channel, types::BlxrCredentials>> {
    let rpc_opts = types::RpcOpts {
        endpoint: String::from(gateway_url),
        disable_auth: false,
        use_tls: true,
        auth_header: String::from(auth_header),
    };
    let url = rpc_opts.endpoint.clone();
    let url_boxed: Box<str> = url.into_boxed_str();
    let url_static: &'static str = Box::leak(url_boxed);
    let channel_result = tonic::transport::Channel::from_static(url_static)
        .connect().await;

    let client = gateway::gateway_client::GatewayClient::with_interceptor(channel_result.unwrap(), types::BlxrCredentials {
        authorization: rpc_opts.auth_header,
    });
    client
}

pub fn public_key_to_address(pk: secp256k1::PublicKey) -> String {
    let serialized = pk.serialize_uncompressed();
    let hash = Keccak256::digest(&serialized[1..]);
    hex::encode(&hash[12..])
}
