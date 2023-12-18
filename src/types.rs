use serde::{Deserialize, Deserializer, Serialize};
use tonic::{Request, Status};
use tonic::service::Interceptor;

#[derive(Clone)]
pub struct BlxrCredentials {
    pub authorization: String,
}

impl Interceptor for BlxrCredentials {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        req.metadata_mut().insert("authorization", self.authorization.parse().unwrap());
        Ok(req)
    }
}

pub struct RpcOpts {
    pub endpoint: String,
    pub disable_auth: bool,
    pub use_tls: bool,
    pub auth_header: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AoriTakeOrderIntent {
    pub(crate) intent_id: String,
    pub take_order_request: TakeOrderRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AoriMakeOrderIntent {
    pub intent_id: String,
    pub make_order: AoriMakeOrder,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AoriMakeOrder {
    pub id: Option<serde_json::Value>,
    pub result: ResultData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultData {
    #[serde(rename = "type")]
    r#type: String,
    pub data: OrderData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderData {
    pub order: AoriOrderDetail,
    #[serde(rename = "orderHash")]
    pub(crate) order_hash: String,
    #[serde(rename = "inputToken")]
    input_token: String,
    #[serde(rename = "outputToken")]
    output_token: String,
    #[serde(rename = "inputAmount")]
    input_amount: String,
    #[serde(rename = "outputAmount")]
    output_amount: String,
    rate: f64,
    #[serde(rename = "chainId")]
    pub(crate) chain_id: i32,
    active: bool,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "lastUpdatedAt")]
    last_updated_at: i64,
    #[serde(rename = "isPublic")]
    is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TakeOrderRequestParams {
    #[serde(rename = "orderId")]
    pub(crate) order_id: String,
    pub(crate) order: AoriOrderDetail,
    #[serde(rename = "seatId")]
    pub(crate) seat_id: i32,
    #[serde(rename = "chainId")]
    pub(crate) chain_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AoriOrderDetail {
    pub parameters: OrderParameters,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TakeOrderRequest {
    pub(crate) id: i32,
    pub(crate) jsonrpc: String,
    pub(crate) method: String,
    pub(crate) params: Vec<TakeOrderRequestParams>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OfferDetail {
    #[serde(rename = "itemType")]
    pub item_type: u8,
    pub token: String,
    #[serde(rename = "identifierOrCriteria")]
    pub identifier_or_criteria: String,
    #[serde(rename = "startAmount")]
    pub start_amount: String,
    #[serde(rename = "endAmount")]
    pub end_amount: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ConsiderationDetail {
    #[serde(rename = "itemType")]
    pub item_type: u8,
    pub token: String,
    #[serde(rename = "identifierOrCriteria")]
    pub identifier_or_criteria: String,
    #[serde(rename = "startAmount")]
    pub start_amount: String,
    #[serde(rename = "endAmount")]
    pub end_amount: String,
    pub recipient: String,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrderParameters {
    pub offerer: String,
    pub zone: String,
    pub offer: Vec<OfferDetail>,
    pub consideration: Vec<ConsiderationDetail>,
    #[serde(rename = "orderType")]
    pub order_type: u8,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "zoneHash")]
    pub zone_hash: String,
    pub salt: String,
    #[serde(rename = "conduitKey")]
    pub conduit_key: String,
    #[serde(rename = "totalOriginalConsiderationItems")]
    pub total_original_consideration_items: i16,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCreationData {
    pub parameters: OrderParameters,
    pub signature: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AoriResponse {
    pub id: Option<u64>,
    pub result: ResultJ,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultJ {
    #[serde(rename = "type")]
    pub result_type: String,
    pub data: OrderCreatedData,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub enum AoriEvent {
    #[serde(rename = "Subscribed")]
    Subscribed(String),

    #[serde(rename = "OrderCreated")]
    OrderCreated(Box<OrderCreatedData>),

}
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct OrderCreatedData {
    pub order: OrderCreationData,
    #[serde(rename = "orderHash")]
    pub order_hash: String,
    #[serde(rename = "inputToken")]
    pub input_token: String,
    #[serde(rename = "outputToken")]
    pub output_token: String,
    #[serde(rename = "inputAmount")]
    pub input_amount: String,
    #[serde(rename = "outputAmount")]
    pub output_amount: String,
    #[serde(rename = "chainId")]
    pub chain_id: i64,
    pub active: bool,
    #[serde(rename = "createdAt")]
    pub created_at: u64,
    #[serde(rename = "lastUpdatedAt")]
    pub last_updated_at: u64,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    pub rate: f64,
}