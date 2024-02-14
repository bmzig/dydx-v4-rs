// We want to allow for non snake case so structs can deserialize properly.
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    subaccount_number: u64,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeightResponse {
    pub height: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FillResponseObject {
    pub id: String,
    pub side: String,
    pub liquidity: String,
    pub r#type: String,
    pub market: String,
    pub market_type: String,
    pub price: String,
    pub size: String,
    pub fee: String,
    pub created_at: String,
    pub created_at_height: String,
    pub order_id: String,
    pub client_metadata: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionResponseObject {
    pub market: String,
    pub status: String,
    pub side: String,
    pub size: String,
    pub max_size: String,
    pub entry_price: String,
    pub realized_pnl: String,
    pub created_at: String,
    pub created_at_height: String,
    pub sum_open: String,
    pub sum_close: String,
    pub net_funding: String,
    pub unrealized_pnl: String,
    pub closed_at: String,
    pub exit_price: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionsResponse {
    pub perpetual_positions: Vec<PerpetualPositionResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPositionResponseObject {
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub asset_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPositionsResponse {
    pub asset_positions: Vec<AssetPositionResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubaccountResponse {
    pub address: String,
    pub subaccount_number: f32,
    pub equity: String,
    pub free_collateral: String,
    pub open_perpetual_positions: PerpetualPositionsResponse,
    pub asset_positions: AssetPositionsResponse,
    pub margin_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub subaccounts: Vec<SubaccountResponse>,
    pub total_trading_rewards: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleResponseObject {
    pub started_at: String,
    pub ticker: String,
    pub resolution: String,
    pub low: String,
    pub high: String,
    pub open: String,
    pub close: String,
    pub base_token_volume: String,
    pub usd_volume: String,
    pub trades: f32,
    pub starting_open_interest: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleResponse {
    pub candles: Vec<CandleResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceResponse {
    pub restricted: bool,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FillResponse {
    pub fills: Vec<FillResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBlockTradingReward {
    pub trading_reward: String,
    pub created_at: String,
    pub created_at_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalBlockTradingRewardsResponse {
    pub rewards: Vec<HistoricalBlockTradingReward>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalFundingResponseObject {
    pub ticker: String,
    pub rate: String,
    pub price: String,
    pub effective_at: String,
    pub effective_at_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalFundingResponse {
    pub historical_funding: Vec<HistoricalFundingResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PnlTicksResponseObject {
    pub id: String,
    pub subaccount_id: String,
    pub equity: String,
    pub total_pnl: String,
    pub net_transfers: String,
    pub created_at: String,
    pub block_height: String,
    pub block_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalPnlResponse {
    pub historical_pnl: Vec<PnlTicksResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTradingRewardAggregation {
    pub trading_reward: String,
    pub started_at: String,
    pub started_at_height: String,
    pub ended_at: String,
    pub ended_at_height: String,
    pub period: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTradingRewardAggregationsResponse {
    pub rewards: Vec<HistoricalTradingRewardAggregation>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookResponsePriceLevel {
    pub price: String,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderbookResponseObject {
    pub bids: Vec<OrderbookResponsePriceLevel>,
    pub asks: Vec<OrderbookResponsePriceLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponseObject {
    pub id: String,
    pub subaccount_id: String,
    pub client_id: String,
    pub clob_pair_id: String,
    pub side: String,
    pub size: String,
    pub total_filled: String,
    pub price: String,
    pub r#type: String,
    pub reduce_only: bool,
    pub order_flags: String,
    pub good_til_block: String,
    pub good_til_block_time: String,
    pub created_at_height: String,
    pub client_metadata: String,
    pub trigger_price: String,
    pub time_in_force: String,
    pub status: String,
    pub post_only: bool,
    pub ticker: String,
    pub updated_at: String,
    pub updated_at_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderResponse {
    orders: Vec<OrderResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualMarketResponseObject {
    pub clobPairId: String,
    pub ticker: String,
    pub status: String,
    pub oraclePrice: String,
    pub priceChange24H: String,
    pub volume24H: String,
    pub trades24H: f32,
    pub nextFundingRate: String,
    pub initialMarginFraction: String,
    pub maintenanceMarginFraction: String,
    pub openInterest: String,
    pub atomicResolution: f32,
    pub quantumConversionExponent: f32,
    pub tickSize: String,
    pub stepSize: String,
    pub stepBaseQuantums: f32,
    pub subticksPerTick: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualMarketResponse {
    pub markets: HashMap<String, PerpetualMarketResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionResponse {
    pub positions: Vec<PerpetualPositionResponseObject>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SparklineResponseObject {
    pub properties: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeResponse {
    pub iso: String,
    pub epoch: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponseObject {
    pub id: String,
    pub side: String,
    pub size: String,
    pub price: String,
    pub r#type: String,
    pub created_at: String,
    pub created_at_height: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeResponse {
    pub trades: Vec<TradeResponseObject>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponseObject {
    pub id: String,
    pub sender: Account,
    pub recipient: Account,
    pub size: String,
    pub created_at: String,
    pub created_at_height: String,
    pub symbol: String,
    pub r#type: String,
    pub transaction_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    pub transfers: Vec<TransferResponseObject>,
}

