// We want to allow for non snake case so structs can deserialize properly.
#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    subaccountNumber: u64,
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
    pub marketType: String,
    pub price: String,
    pub size: String,
    pub fee: String,
    pub createdAt: String,
    pub createdAtHeight: String,
    pub orderId: String,
    pub clientMetadata: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionResponseObject {
    pub market: String,
    pub status: String,
    pub side: String,
    pub size: String,
    pub maxSize: String,
    pub entryPrice: String,
    pub realizedPnl: String,
    pub createdAt: String,
    pub createdAtHeight: String,
    pub sumOpen: String,
    pub sumClose: String,
    pub netFunding: String,
    pub unrealizedPnl: String,
    pub closedAt: String,
    pub exitPrice: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerpetualPositionsResponse {
    pub perpetualPositions: HashMap<String, PerpetualPositionResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPositionResponseObject {
    pub symbol: String,
    pub side: String,
    pub size: String,
    pub assetId: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPositionsResponse {
    pub assetPositions: HashMap<String, AssetPositionResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubaccountResponse {
    pub address: String,
    pub subaccountNumber: f32,
    pub equity: String,
    pub freeCollateral: String,
    pub openPerpetualPositions: HashMap<String, PerpetualPositionResponseObject>,
    pub assetPositions: HashMap<String, AssetPositionResponseObject>,
    pub marginEnabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressResponse {
    pub subaccounts: Vec<SubaccountResponse>,
    pub totalTradingRewards: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CandleResponseObject {
    pub startedAt: String,
    pub ticker: String,
    pub resolution: String,
    pub low: String,
    pub high: String,
    pub open: String,
    pub close: String,
    pub baseTokenVolume: String,
    pub usdVolume: String,
    pub trades: f32,
    pub startingOpenInterest: String,
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
    pub tradingReward: String,
    pub createdAt: String,
    pub createdAtHeight: String,
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
    pub effectiveAt: String,
    pub effectiveAtHeight: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalFundingResponse {
    pub historicalFunding: Vec<HistoricalFundingResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PnlTicksResponseObject {
    pub id: String,
    pub subaccount_id: String,
    pub equity: String,
    pub totalPnl: String,
    pub netTransfers: String,
    pub createdAt: String,
    pub blockHeight: String,
    pub blockTime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalPnlResponse {
    pub historicalPnl: Vec<PnlTicksResponseObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoricalTradingRewardAggregation {
    pub tradingReward: String,
    pub startedAt: String,
    pub startedAtHeight: String,
    pub endedAt: String,
    pub endedAtHeight: String,
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
    pub subaccountId: String,
    pub clientId: String,
    pub clobPairId: String,
    pub side: String,
    pub size: String,
    pub totalFilled: String,
    pub price: String,
    pub r#type: String,
    pub reduceOnly: bool,
    pub orderFlags: String,
    pub goodTilBlock: String,
    pub goodTilBlock_time: String,
    pub createdAtHeight: String,
    pub clientMetadata: String,
    pub triggerPrice: String,
    pub timeInForce: String,
    pub status: String,
    pub postOnly: bool,
    pub ticker: String,
    pub updatedAt: String,
    pub updatedAtHeight: String,
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
    pub createdAt: String,
    pub createdAtHeight: String,
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
    pub createdAt: String,
    pub createdAtHeight: String,
    pub symbol: String,
    pub r#type: String,
    pub transactionHash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    pub transfers: Vec<TransferResponseObject>,
}

