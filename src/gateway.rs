#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxLogs {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub data: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub transaction_index: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub log_index: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub removed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxReceiptsRequest {
    #[prost(string, repeated, tag = "1")]
    pub includes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(string, tag = "2")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxReceiptsReply {
    #[prost(string, tag = "1")]
    pub bloc_k_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub cumulative_gas_used: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub effective_gas_used: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub gas_used: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "8")]
    pub logs: ::prost::alloc::vec::Vec<TxLogs>,
    #[prost(string, tag = "9")]
    pub logs_bloom: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub transaction_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub transaction_index: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub txs_count: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallParams {
    #[prost(map = "string, string", tag = "1")]
    pub params: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthOnBlockRequest {
    #[prost(string, repeated, tag = "1")]
    pub includes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub call_params: ::prost::alloc::vec::Vec<CallParams>,
    #[deprecated]
    #[prost(string, tag = "3")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthOnBlockReply {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub response: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub block_height: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrSubmitBundleRequest {
    #[prost(map = "string, string", tag = "1")]
    pub mev_builders: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, repeated, tag = "2")]
    pub transactions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub min_timestamp: i64,
    #[prost(int64, tag = "5")]
    pub max_timestamp: i64,
    #[prost(string, repeated, tag = "6")]
    pub reverting_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "7")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(int64, tag = "8")]
    pub bundle_price: i64,
    #[prost(bool, tag = "9")]
    pub enforce_payout: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrSubmitBundleReply {
    #[prost(string, tag = "1")]
    pub bundle_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxsRequest {
    #[prost(string, tag = "1")]
    pub filters: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub includes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(string, tag = "3")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tx {
    #[prost(bytes = "vec", tag = "12")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "14")]
    pub local_region: bool,
    #[prost(int64, tag = "15")]
    pub time: i64,
    #[prost(bytes = "vec", tag = "16")]
    pub raw_tx: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessTuple {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub storage_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxsReply {
    #[prost(message, repeated, tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<Tx>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksRequest {
    #[prost(string, repeated, tag = "1")]
    pub includes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[deprecated]
    #[prost(string, tag = "2")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(string, tag = "1")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sha3_uncles: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub miner: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub state_root: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub transactions_root: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub receipts_root: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub logs_bloom: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub difficulty: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub number: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub gas_used: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub extra_data: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub mix_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "15")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub base_fee_per_gas: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub withdrawals_root: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureValidatorInfo {
    #[prost(string, tag = "1")]
    pub block_height: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub wallet_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub accessible: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlocksReply {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "2")]
    pub subscription_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub header: ::core::option::Option<BlockHeader>,
    #[prost(message, repeated, tag = "4")]
    pub future_validator_info: ::prost::alloc::vec::Vec<FutureValidatorInfo>,
    #[prost(message, repeated, tag = "5")]
    pub transaction: ::prost::alloc::vec::Vec<Tx>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectInboundPeerRequest {
    #[prost(string, tag = "1")]
    pub peer_ip: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub peer_port: i64,
    #[prost(string, tag = "3")]
    pub public_key: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisconnectInboundPeerReply {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionsRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tier: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub feed_name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "4")]
    pub network: u32,
    #[prost(string, tag = "5")]
    pub remote_addr: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub include: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub filter: ::prost::alloc::string::String,
    #[prost(uint64, tag = "8")]
    pub age: u64,
    #[prost(uint64, tag = "9")]
    pub messages_sent: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionsReply {
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::prost::alloc::vec::Vec<Subscription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionReply {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub build_date: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeersRequest {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "2")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RateSnapshot {
    #[prost(int64, tag = "1")]
    pub five_minute: i64,
    #[prost(int64, tag = "2")]
    pub one_hour: i64,
    #[prost(int64, tag = "3")]
    pub one_day: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(string, tag = "1")]
    pub ip: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub state: ::prost::alloc::string::String,
    #[prost(uint32, tag = "5")]
    pub network: u32,
    #[prost(bool, tag = "6")]
    pub initiator: bool,
    #[prost(int64, tag = "7")]
    pub min_us_from_peer: i64,
    #[prost(int64, tag = "8")]
    pub min_us_to_peer: i64,
    #[prost(int64, tag = "9")]
    pub slow_traffic_count: i64,
    #[prost(int64, tag = "10")]
    pub min_us_round_trip: i64,
    #[prost(string, tag = "11")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub account_tier: ::prost::alloc::string::String,
    #[prost(int64, tag = "13")]
    pub port: i64,
    #[prost(bool, tag = "14")]
    pub disabled: bool,
    #[prost(string, tag = "15")]
    pub mev_miner: ::prost::alloc::string::String,
    /// Deprecated
    #[prost(string, tag = "16")]
    pub mev_builder: ::prost::alloc::string::String,
    #[prost(uint32, tag = "17")]
    pub capability: u32,
    #[prost(int64, tag = "18")]
    pub unpaid_tx_burst_limit: i64,
    #[prost(int64, tag = "19")]
    pub paid_tx_burst_limit: i64,
    #[prost(message, optional, tag = "20")]
    pub unpaid_tx_burst_limit_excess: ::core::option::Option<RateSnapshot>,
    #[prost(message, optional, tag = "21")]
    pub paid_tx_burst_limit_excess: ::core::option::Option<RateSnapshot>,
    #[prost(message, optional, tag = "22")]
    pub paid_tx_throughput: ::core::option::Option<RateSnapshot>,
    #[prost(message, optional, tag = "23")]
    pub unpaid_tx_throughput: ::core::option::Option<RateSnapshot>,
    #[prost(string, tag = "24")]
    pub trusted: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "25")]
    pub mev_builders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int64, tag = "26")]
    pub new_txs: i64,
    #[prost(int64, tag = "27")]
    pub already_seen_txs: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeersReply {
    #[prost(message, repeated, tag = "1")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendTxRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BxTransaction {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "2")]
    pub short_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(message, optional, tag = "3")]
    pub add_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBxTransactionRequest {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBxTransactionResponse {
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<BxTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStoreRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStoreNetworkData {
    #[prost(uint64, tag = "4")]
    pub network: u64,
    #[prost(uint64, tag = "1")]
    pub tx_count: u64,
    #[prost(uint64, tag = "2")]
    pub short_id_count: u64,
    #[prost(message, optional, tag = "3")]
    pub oldest_tx: ::core::option::Option<BxTransaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxStoreReply {
    #[prost(uint64, tag = "1")]
    pub tx_count: u64,
    #[prost(uint64, tag = "2")]
    pub short_id_count: u64,
    #[prost(message, repeated, tag = "3")]
    pub network_data: ::prost::alloc::vec::Vec<TxStoreNetworkData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxAndSender {
    #[prost(string, tag = "1")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrBatchTxRequest {
    #[prost(message, repeated, tag = "1")]
    pub transactions_and_senders: ::prost::alloc::vec::Vec<TxAndSender>,
    #[prost(bool, tag = "2")]
    pub nonce_monitoring: bool,
    #[prost(bool, tag = "3")]
    pub next_validator: bool,
    #[prost(int32, tag = "4")]
    pub fallback: i32,
    #[prost(bool, tag = "5")]
    pub validators_only: bool,
    #[prost(bool, tag = "6")]
    pub node_validation: bool,
    #[prost(int64, tag = "7")]
    pub sending_time: i64,
    #[deprecated]
    #[prost(string, tag = "8")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub frontrunning_protection: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrTxRequest {
    #[prost(string, tag = "1")]
    pub transaction: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub nonce_monitoring: bool,
    #[prost(bool, tag = "3")]
    pub next_validator: bool,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub validators_only: bool,
    #[prost(int32, tag = "6")]
    pub fallback: i32,
    #[prost(bool, tag = "7")]
    pub node_validation: bool,
    #[prost(bool, tag = "8")]
    pub frontrunning_protection: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrTxReply {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxIndex {
    #[prost(int32, tag = "1")]
    pub idx: i32,
    #[prost(string, tag = "2")]
    pub tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorIndex {
    #[prost(int32, tag = "1")]
    pub idx: i32,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlxrBatchTxReply {
    #[prost(message, repeated, tag = "1")]
    pub tx_hashes: ::prost::alloc::vec::Vec<TxIndex>,
    #[prost(message, repeated, tag = "2")]
    pub tx_errors: ::prost::alloc::vec::Vec<ErrorIndex>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub expire_date: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueuesStats {
    #[prost(uint64, tag = "1")]
    pub txs_queue_count: u64,
    #[prost(uint64, tag = "2")]
    pub txs_order_queue_count: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePerformance {
    #[prost(string, tag = "1")]
    pub since: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub new_blocks_received_from_blockchain_node: u32,
    #[prost(uint32, tag = "3")]
    pub new_blocks_received_from_bdn: u32,
    #[prost(uint32, tag = "4")]
    pub new_blocks_seen: u32,
    #[prost(uint32, tag = "5")]
    pub new_block_messages_from_blockchain_node: u32,
    #[prost(uint32, tag = "6")]
    pub new_block_announcements_from_blockchain_node: u32,
    #[prost(uint32, tag = "7")]
    pub new_tx_received_from_blockchain_node: u32,
    #[prost(uint32, tag = "8")]
    pub new_tx_received_from_bdn: u32,
    #[prost(uint32, tag = "9")]
    pub tx_sent_to_node: u32,
    #[prost(uint32, tag = "10")]
    pub duplicate_tx_from_node: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WsConnStatus {
    #[prost(string, tag = "1")]
    pub addr: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub conn_status: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub sync_status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConnStatus {
    #[prost(string, tag = "1")]
    pub conn_status: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub ws_connection: ::core::option::Option<WsConnStatus>,
    #[prost(message, optional, tag = "3")]
    pub node_performance: ::core::option::Option<NodePerformance>,
    /// Inbound is not in use and kept for back compatibility
    #[prost(bool, tag = "4")]
    pub inbound: bool,
    #[prost(bool, tag = "5")]
    pub is_connected: bool,
    #[prost(bool, tag = "6")]
    pub dynamic: bool,
    #[prost(int64, tag = "7")]
    pub version: i64,
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub connected_at: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BdnConnStatus {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connected_at: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub latency: ::core::option::Option<ConnectionLatency>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionLatency {
    #[prost(int64, tag = "7")]
    pub min_ms_from_peer: i64,
    #[prost(int64, tag = "8")]
    pub min_ms_to_peer: i64,
    #[prost(int64, tag = "9")]
    pub slow_traffic_count: i64,
    #[prost(int64, tag = "10")]
    pub min_ms_round_trip: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayInfo {
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub ip_address: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub time_started: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub continent: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub country: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub startup_params: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub gateway_public_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, optional, tag = "2")]
    pub gateway_info: ::core::option::Option<GatewayInfo>,
    #[prost(map = "string, message", tag = "3")]
    pub nodes: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        NodeConnStatus,
    >,
    #[prost(map = "string, message", tag = "4")]
    pub relays: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        BdnConnStatus,
    >,
    #[prost(message, optional, tag = "1")]
    pub account_info: ::core::option::Option<AccountInfo>,
    #[prost(message, optional, tag = "5")]
    pub queue_stats: ::core::option::Option<QueuesStats>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxResult {
    #[prost(string, tag = "1")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx_contents: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub local_region: bool,
    #[prost(string, tag = "4")]
    pub time: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub raw_tx: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxHashListRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub tx_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortIdListReply {
    #[prost(uint32, repeated, tag = "1")]
    pub short_i_ds: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShortIdListRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(uint32, repeated, tag = "2")]
    pub short_i_ds: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxListReply {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposedBlockRequest {
    #[deprecated]
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_http_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub block_number: u64,
    #[prost(string, tag = "5")]
    pub prev_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub block_reward: ::prost::alloc::string::String,
    #[prost(uint64, tag = "7")]
    pub gas_limit: u64,
    #[prost(uint64, tag = "8")]
    pub gas_used: u64,
    #[prost(message, repeated, tag = "9")]
    pub payload: ::prost::alloc::vec::Vec<CompressTx>,
    #[prost(bool, tag = "10")]
    pub process_blocks_on_gateway: bool,
    #[prost(string, tag = "11")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", repeated, tag = "12")]
    pub un_reverted_hashes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompressTx {
    #[prost(bytes = "vec", tag = "1")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "2")]
    pub short_id: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposedBlockReply {
    #[prost(string, tag = "1")]
    pub validator_reply: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub validator_reply_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfoRequest {
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub start_sending_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag = "3")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockInfoReply {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposedBlockStatsRequest {
    #[prost(string, tag = "1")]
    pub auth_header: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub block_number: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposedBlockStatsReply {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub sending_duration: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag = "3")]
    pub received_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "4")]
    pub sent_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "5")]
    pub validator_reply: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub validator_reply_time: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitBeaconBlockSszRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub block_ssz: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitBeaconBlockSszReply {}
/// Intent Gateway Messages
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitIntentRequest {
    /// PubKey
    #[prost(string, tag = "1")]
    pub dapp_address: ::prost::alloc::string::String,
    /// The intent payload
    #[prost(bytes = "vec", tag = "2")]
    pub intent: ::prost::alloc::vec::Vec<u8>,
    /// Keccak256Hash of the Intent bytes
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// ECDSA signature of hash
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub expiry_duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitIntentReply {
    #[prost(string, tag = "1")]
    pub intent_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelIntentRequest {
    /// Address of the user submitting the intent
    #[prost(string, tag = "1")]
    pub dapp_address: ::prost::alloc::string::String,
    ///
    #[prost(string, tag = "2")]
    pub intent_id: ::prost::alloc::string::String,
    /// Keccak256Hash of the intentId bytes
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// ECDSA signature
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "5")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelIntentReply {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitIntentSolutionRequest {
    #[prost(string, tag = "1")]
    pub solver_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub intent_id: ::prost::alloc::string::String,
    /// The solution payload
    #[prost(bytes = "vec", tag = "3")]
    pub intent_solution: ::prost::alloc::vec::Vec<u8>,
    /// Keccak256Hash of the IntentSolution bytes
    #[prost(bytes = "vec", tag = "4")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// ECDSA signature of the hash
    #[prost(bytes = "vec", tag = "5")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "6")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitIntentSolutionReply {
    #[prost(string, tag = "1")]
    pub solution_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentsRequest {
    #[prost(string, tag = "1")]
    pub solver_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub filters: ::prost::alloc::string::String,
    /// Keccak256Hash of the solverAddress bytes
    #[prost(bytes = "vec", tag = "3")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// ECDSA signature
    #[prost(bytes = "vec", tag = "4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "5")]
    pub from_timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag = "6")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentsReply {
    /// GUID expected
    #[prost(string, tag = "1")]
    pub dapp_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub intent_id: ::prost::alloc::string::String,
    /// The intent payload
    #[prost(bytes = "vec", tag = "3")]
    pub intent: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentSolutionsRequest {
    #[prost(string, tag = "1")]
    pub dapp_address: ::prost::alloc::string::String,
    /// Keccak256Hash of the dappAddress bytes
    #[prost(bytes = "vec", tag = "2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    /// ECDSA signature of the hash
    #[prost(bytes = "vec", tag = "3")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub auth_header: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentSolutionsReply {
    #[prost(string, tag = "1")]
    pub intent_id: ::prost::alloc::string::String,
    /// The solution payload
    #[prost(bytes = "vec", tag = "2")]
    pub intent_solution: ::prost::alloc::vec::Vec<u8>,
}
/// Generated client implementations.
pub mod gateway_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct GatewayClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GatewayClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> GatewayClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GatewayClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            GatewayClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn blxr_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::BlxrTxRequest>,
        ) -> std::result::Result<tonic::Response<super::BlxrTxReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/BlxrTx");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "BlxrTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blxr_batch_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::BlxrBatchTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlxrBatchTxReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/BlxrBatchTX",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "BlxrBatchTX"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn peers(
            &mut self,
            request: impl tonic::IntoRequest<super::PeersRequest>,
        ) -> std::result::Result<tonic::Response<super::PeersReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/Peers");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "Peers"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tx_store_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::TxStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::TxStoreReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/TxStoreSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "TxStoreSummary"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBxTransactionResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/GetTx");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "GetTx"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/Stop");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "Stop"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn version(
            &mut self,
            request: impl tonic::IntoRequest<super::VersionRequest>,
        ) -> std::result::Result<tonic::Response<super::VersionReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/Version");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "Version"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/Status");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "Status"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubscriptionsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/Subscriptions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "Subscriptions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn disconnect_inbound_peer(
            &mut self,
            request: impl tonic::IntoRequest<super::DisconnectInboundPeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisconnectInboundPeerReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/DisconnectInboundPeer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "DisconnectInboundPeer"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::TxsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TxsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/NewTxs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "NewTxs"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn pending_txs(
            &mut self,
            request: impl tonic::IntoRequest<super::TxsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TxsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/PendingTxs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "PendingTxs"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn new_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::BlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BlocksReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/NewBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "NewBlocks"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn bdn_blocks(
            &mut self,
            request: impl tonic::IntoRequest<super::BlocksRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::BlocksReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/BdnBlocks",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "BdnBlocks"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn eth_on_block(
            &mut self,
            request: impl tonic::IntoRequest<super::EthOnBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::EthOnBlockReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/EthOnBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "EthOnBlock"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn tx_receipts(
            &mut self,
            request: impl tonic::IntoRequest<super::TxReceiptsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TxReceiptsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/TxReceipts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "TxReceipts"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn short_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::TxHashListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShortIdListReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/ShortIDs");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "ShortIDs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn proposed_block(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProposedBlockReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/ProposedBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "ProposedBlock"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn txs_from_short_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::ShortIdListRequest>,
        ) -> std::result::Result<tonic::Response<super::TxListReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/TxsFromShortIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "TxsFromShortIDs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn block_info(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockInfoReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/BlockInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "BlockInfo"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn proposed_block_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::ProposedBlockStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProposedBlockStatsReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/ProposedBlockStats",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "ProposedBlockStats"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn blxr_submit_bundle(
            &mut self,
            request: impl tonic::IntoRequest<super::BlxrSubmitBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlxrSubmitBundleReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/BlxrSubmitBundle",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "BlxrSubmitBundle"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_beacon_block_ssz(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitBeaconBlockSszRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitBeaconBlockSszReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/SubmitBeaconBlockSSZ",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "SubmitBeaconBlockSSZ"));
            self.inner.unary(req, path, codec).await
        }
        /// Intent Gateway functions
        pub async fn submit_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitIntentReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/SubmitIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "SubmitIntent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelIntentReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/CancelIntent",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "CancelIntent"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_intent_solution(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitIntentSolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitIntentSolutionReply>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/SubmitIntentSolution",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "SubmitIntentSolution"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn intents(
            &mut self,
            request: impl tonic::IntoRequest<super::IntentsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::IntentsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/gateway.Gateway/Intents");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("gateway.Gateway", "Intents"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn intent_solutions(
            &mut self,
            request: impl tonic::IntoRequest<super::IntentSolutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::IntentSolutionsReply>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/gateway.Gateway/IntentSolutions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("gateway.Gateway", "IntentSolutions"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod gateway_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with GatewayServer.
    #[async_trait]
    pub trait Gateway: Send + Sync + 'static {
        async fn blxr_tx(
            &self,
            request: tonic::Request<super::BlxrTxRequest>,
        ) -> std::result::Result<tonic::Response<super::BlxrTxReply>, tonic::Status>;
        async fn blxr_batch_tx(
            &self,
            request: tonic::Request<super::BlxrBatchTxRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlxrBatchTxReply>,
            tonic::Status,
        >;
        async fn peers(
            &self,
            request: tonic::Request<super::PeersRequest>,
        ) -> std::result::Result<tonic::Response<super::PeersReply>, tonic::Status>;
        async fn tx_store_summary(
            &self,
            request: tonic::Request<super::TxStoreRequest>,
        ) -> std::result::Result<tonic::Response<super::TxStoreReply>, tonic::Status>;
        async fn get_tx(
            &self,
            request: tonic::Request<super::GetBxTransactionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBxTransactionResponse>,
            tonic::Status,
        >;
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> std::result::Result<tonic::Response<super::StopReply>, tonic::Status>;
        async fn version(
            &self,
            request: tonic::Request<super::VersionRequest>,
        ) -> std::result::Result<tonic::Response<super::VersionReply>, tonic::Status>;
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        async fn subscriptions(
            &self,
            request: tonic::Request<super::SubscriptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubscriptionsReply>,
            tonic::Status,
        >;
        async fn disconnect_inbound_peer(
            &self,
            request: tonic::Request<super::DisconnectInboundPeerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DisconnectInboundPeerReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the NewTxs method.
        type NewTxsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::TxsReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn new_txs(
            &self,
            request: tonic::Request<super::TxsRequest>,
        ) -> std::result::Result<tonic::Response<Self::NewTxsStream>, tonic::Status>;
        /// Server streaming response type for the PendingTxs method.
        type PendingTxsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::TxsReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn pending_txs(
            &self,
            request: tonic::Request<super::TxsRequest>,
        ) -> std::result::Result<tonic::Response<Self::PendingTxsStream>, tonic::Status>;
        /// Server streaming response type for the NewBlocks method.
        type NewBlocksStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BlocksReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn new_blocks(
            &self,
            request: tonic::Request<super::BlocksRequest>,
        ) -> std::result::Result<tonic::Response<Self::NewBlocksStream>, tonic::Status>;
        /// Server streaming response type for the BdnBlocks method.
        type BdnBlocksStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::BlocksReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn bdn_blocks(
            &self,
            request: tonic::Request<super::BlocksRequest>,
        ) -> std::result::Result<tonic::Response<Self::BdnBlocksStream>, tonic::Status>;
        /// Server streaming response type for the EthOnBlock method.
        type EthOnBlockStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::EthOnBlockReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn eth_on_block(
            &self,
            request: tonic::Request<super::EthOnBlockRequest>,
        ) -> std::result::Result<tonic::Response<Self::EthOnBlockStream>, tonic::Status>;
        /// Server streaming response type for the TxReceipts method.
        type TxReceiptsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::TxReceiptsReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn tx_receipts(
            &self,
            request: tonic::Request<super::TxReceiptsRequest>,
        ) -> std::result::Result<tonic::Response<Self::TxReceiptsStream>, tonic::Status>;
        async fn short_i_ds(
            &self,
            request: tonic::Request<super::TxHashListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShortIdListReply>,
            tonic::Status,
        >;
        async fn proposed_block(
            &self,
            request: tonic::Request<super::ProposedBlockRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProposedBlockReply>,
            tonic::Status,
        >;
        async fn txs_from_short_i_ds(
            &self,
            request: tonic::Request<super::ShortIdListRequest>,
        ) -> std::result::Result<tonic::Response<super::TxListReply>, tonic::Status>;
        async fn block_info(
            &self,
            request: tonic::Request<super::BlockInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::BlockInfoReply>, tonic::Status>;
        async fn proposed_block_stats(
            &self,
            request: tonic::Request<super::ProposedBlockStatsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ProposedBlockStatsReply>,
            tonic::Status,
        >;
        async fn blxr_submit_bundle(
            &self,
            request: tonic::Request<super::BlxrSubmitBundleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BlxrSubmitBundleReply>,
            tonic::Status,
        >;
        async fn submit_beacon_block_ssz(
            &self,
            request: tonic::Request<super::SubmitBeaconBlockSszRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitBeaconBlockSszReply>,
            tonic::Status,
        >;
        /// Intent Gateway functions
        async fn submit_intent(
            &self,
            request: tonic::Request<super::SubmitIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitIntentReply>,
            tonic::Status,
        >;
        async fn cancel_intent(
            &self,
            request: tonic::Request<super::CancelIntentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelIntentReply>,
            tonic::Status,
        >;
        async fn submit_intent_solution(
            &self,
            request: tonic::Request<super::SubmitIntentSolutionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SubmitIntentSolutionReply>,
            tonic::Status,
        >;
        /// Server streaming response type for the Intents method.
        type IntentsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::IntentsReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn intents(
            &self,
            request: tonic::Request<super::IntentsRequest>,
        ) -> std::result::Result<tonic::Response<Self::IntentsStream>, tonic::Status>;
        /// Server streaming response type for the IntentSolutions method.
        type IntentSolutionsStream: tonic::codegen::tokio_stream::Stream<
                Item = std::result::Result<super::IntentSolutionsReply, tonic::Status>,
            >
            + Send
            + 'static;
        async fn intent_solutions(
            &self,
            request: tonic::Request<super::IntentSolutionsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::IntentSolutionsStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct GatewayServer<T: Gateway> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Gateway> GatewayServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GatewayServer<T>
    where
        T: Gateway,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/gateway.Gateway/BlxrTx" => {
                    #[allow(non_camel_case_types)]
                    struct BlxrTxSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::BlxrTxRequest>
                    for BlxrTxSvc<T> {
                        type Response = super::BlxrTxReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlxrTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::blxr_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlxrTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/BlxrBatchTX" => {
                    #[allow(non_camel_case_types)]
                    struct BlxrBatchTXSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::BlxrBatchTxRequest>
                    for BlxrBatchTXSvc<T> {
                        type Response = super::BlxrBatchTxReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlxrBatchTxRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::blxr_batch_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlxrBatchTXSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Peers" => {
                    #[allow(non_camel_case_types)]
                    struct PeersSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::PeersRequest>
                    for PeersSvc<T> {
                        type Response = super::PeersReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PeersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::peers(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PeersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/TxStoreSummary" => {
                    #[allow(non_camel_case_types)]
                    struct TxStoreSummarySvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::TxStoreRequest>
                    for TxStoreSummarySvc<T> {
                        type Response = super::TxStoreReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxStoreRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::tx_store_summary(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxStoreSummarySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/GetTx" => {
                    #[allow(non_camel_case_types)]
                    struct GetTxSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::GetBxTransactionRequest>
                    for GetTxSvc<T> {
                        type Response = super::GetBxTransactionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBxTransactionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::get_tx(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetTxSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::StopRequest>
                    for StopSvc<T> {
                        type Response = super::StopReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::stop(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Version" => {
                    #[allow(non_camel_case_types)]
                    struct VersionSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::VersionRequest>
                    for VersionSvc<T> {
                        type Response = super::VersionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VersionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::version(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VersionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::StatusRequest>
                    for StatusSvc<T> {
                        type Response = super::StatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::status(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Subscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct SubscriptionsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::SubscriptionsRequest>
                    for SubscriptionsSvc<T> {
                        type Response = super::SubscriptionsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscriptionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::subscriptions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscriptionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/DisconnectInboundPeer" => {
                    #[allow(non_camel_case_types)]
                    struct DisconnectInboundPeerSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::DisconnectInboundPeerRequest>
                    for DisconnectInboundPeerSvc<T> {
                        type Response = super::DisconnectInboundPeerReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisconnectInboundPeerRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::disconnect_inbound_peer(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisconnectInboundPeerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/NewTxs" => {
                    #[allow(non_camel_case_types)]
                    struct NewTxsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::TxsRequest>
                    for NewTxsSvc<T> {
                        type Response = super::TxsReply;
                        type ResponseStream = T::NewTxsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::new_txs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewTxsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/PendingTxs" => {
                    #[allow(non_camel_case_types)]
                    struct PendingTxsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::TxsRequest>
                    for PendingTxsSvc<T> {
                        type Response = super::TxsReply;
                        type ResponseStream = T::PendingTxsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::pending_txs(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PendingTxsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/NewBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct NewBlocksSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::BlocksRequest>
                    for NewBlocksSvc<T> {
                        type Response = super::BlocksReply;
                        type ResponseStream = T::NewBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::new_blocks(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/BdnBlocks" => {
                    #[allow(non_camel_case_types)]
                    struct BdnBlocksSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::BlocksRequest>
                    for BdnBlocksSvc<T> {
                        type Response = super::BlocksReply;
                        type ResponseStream = T::BdnBlocksStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlocksRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::bdn_blocks(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BdnBlocksSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/EthOnBlock" => {
                    #[allow(non_camel_case_types)]
                    struct EthOnBlockSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::EthOnBlockRequest>
                    for EthOnBlockSvc<T> {
                        type Response = super::EthOnBlockReply;
                        type ResponseStream = T::EthOnBlockStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EthOnBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::eth_on_block(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EthOnBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/TxReceipts" => {
                    #[allow(non_camel_case_types)]
                    struct TxReceiptsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::TxReceiptsRequest>
                    for TxReceiptsSvc<T> {
                        type Response = super::TxReceiptsReply;
                        type ResponseStream = T::TxReceiptsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxReceiptsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::tx_receipts(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxReceiptsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/ShortIDs" => {
                    #[allow(non_camel_case_types)]
                    struct ShortIDsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::TxHashListRequest>
                    for ShortIDsSvc<T> {
                        type Response = super::ShortIdListReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TxHashListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::short_i_ds(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShortIDsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/ProposedBlock" => {
                    #[allow(non_camel_case_types)]
                    struct ProposedBlockSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::ProposedBlockRequest>
                    for ProposedBlockSvc<T> {
                        type Response = super::ProposedBlockReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProposedBlockRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::proposed_block(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposedBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/TxsFromShortIDs" => {
                    #[allow(non_camel_case_types)]
                    struct TxsFromShortIDsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::ShortIdListRequest>
                    for TxsFromShortIDsSvc<T> {
                        type Response = super::TxListReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShortIdListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::txs_from_short_i_ds(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TxsFromShortIDsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/BlockInfo" => {
                    #[allow(non_camel_case_types)]
                    struct BlockInfoSvc<T: Gateway>(pub Arc<T>);
                    impl<T: Gateway> tonic::server::UnaryService<super::BlockInfoRequest>
                    for BlockInfoSvc<T> {
                        type Response = super::BlockInfoReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::block_info(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/ProposedBlockStats" => {
                    #[allow(non_camel_case_types)]
                    struct ProposedBlockStatsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::ProposedBlockStatsRequest>
                    for ProposedBlockStatsSvc<T> {
                        type Response = super::ProposedBlockStatsReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProposedBlockStatsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::proposed_block_stats(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposedBlockStatsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/BlxrSubmitBundle" => {
                    #[allow(non_camel_case_types)]
                    struct BlxrSubmitBundleSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::BlxrSubmitBundleRequest>
                    for BlxrSubmitBundleSvc<T> {
                        type Response = super::BlxrSubmitBundleReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlxrSubmitBundleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::blxr_submit_bundle(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlxrSubmitBundleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/SubmitBeaconBlockSSZ" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitBeaconBlockSSZSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::SubmitBeaconBlockSszRequest>
                    for SubmitBeaconBlockSSZSvc<T> {
                        type Response = super::SubmitBeaconBlockSszReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitBeaconBlockSszRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::submit_beacon_block_ssz(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitBeaconBlockSSZSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/SubmitIntent" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitIntentSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::SubmitIntentRequest>
                    for SubmitIntentSvc<T> {
                        type Response = super::SubmitIntentReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitIntentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::submit_intent(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/CancelIntent" => {
                    #[allow(non_camel_case_types)]
                    struct CancelIntentSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::CancelIntentRequest>
                    for CancelIntentSvc<T> {
                        type Response = super::CancelIntentReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelIntentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::cancel_intent(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/SubmitIntentSolution" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitIntentSolutionSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::UnaryService<super::SubmitIntentSolutionRequest>
                    for SubmitIntentSolutionSvc<T> {
                        type Response = super::SubmitIntentSolutionReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitIntentSolutionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::submit_intent_solution(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitIntentSolutionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/Intents" => {
                    #[allow(non_camel_case_types)]
                    struct IntentsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<super::IntentsRequest>
                    for IntentsSvc<T> {
                        type Response = super::IntentsReply;
                        type ResponseStream = T::IntentsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IntentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::intents(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IntentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/gateway.Gateway/IntentSolutions" => {
                    #[allow(non_camel_case_types)]
                    struct IntentSolutionsSvc<T: Gateway>(pub Arc<T>);
                    impl<
                        T: Gateway,
                    > tonic::server::ServerStreamingService<
                        super::IntentSolutionsRequest,
                    > for IntentSolutionsSvc<T> {
                        type Response = super::IntentSolutionsReply;
                        type ResponseStream = T::IntentSolutionsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IntentSolutionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Gateway>::intent_solutions(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IntentSolutionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Gateway> Clone for GatewayServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Gateway> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Gateway> tonic::server::NamedService for GatewayServer<T> {
        const NAME: &'static str = "gateway.Gateway";
    }
}
