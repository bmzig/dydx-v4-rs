use reqwest::Client;
use concat_string::concat_string;

use crate::{
    constants::*,
    client::{
        IndexerClient,
        responses::*,
    },
};

impl IndexerClient {

    // Initialize
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn mainnet() -> Self {
        Self {
            endpoint: M_IAPI.to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            endpoint: T_IAPI.to_string(),
        }
    }

    // API
    // Indexer Status

    pub async fn get_height(&self) -> anyhow::Result<HeightResponse> {
        let url = concat_string!(self.endpoint, "/height/");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    // Markets
    pub async fn list_perpetual_markets(&self) -> anyhow::Result<PerpetualMarketResponse> {
        let url = concat_string!(self.endpoint, "/perpetualMarkets?limit=100");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    pub async fn get_perpetual_market<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<PerpetualMarketResponseObject> {
        let url = concat_string!(self.endpoint, "/orderbooks/perpetualMarket/", ticker);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn list_sparklines(&self) -> anyhow::Result<SparklineResponseObject> {
        let url = concat_string!(self.endpoint, "/sparklines?timePeriod=SEVEN_DAYS");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_orderbook<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<OrderbookResponseObject> {
        let url = concat_string!(self.endpoint, "/orderbooks/perpetualMarket/", ticker);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_trades<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<TradeResponse> {
        let url = concat_string!(self.endpoint, "/trades/perpetualMarket/", ticker, "?limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_historical_funding<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<HistoricalFundingResponse> {
        let url = concat_string!(self.endpoint, "/historicalFunding/", ticker, "?limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_candles<T: AsRef<str>>(&self, ticker: T, resolution: T) -> anyhow::Result<CandleResponse> {
        let url = concat_string!(self.endpoint, "/candles/perpetualMarkets/", ticker, "?resolution=", resolution, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    // Subaccounts

    pub async fn get_address<T: AsRef<str>>(&self, address: T) -> anyhow::Result<AddressResponse> {
        let url = concat_string!(self.endpoint, "/addresses/", address);
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }
    
    pub async fn get_subaccount<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<SubaccountResponse> {
        let url = concat_string!(self.endpoint, "/addresses/", address, "/subaccountNumber/", subaccount_number);
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    pub async fn get_asset_positions<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<AssetPositionsResponse> {
        let url = concat_string!(self.endpoint, "/assetPositions?address=", address, "&subaccountNumber=", subaccount_number);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_perpetual_positions<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<PerpetualPositionResponse> {
        let url = concat_string!(self.endpoint, "/perpetualPositions?address=", address, "&subaccountNumber=", subaccount_number, "&status=OPEN&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_orders<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<OrderResponseObject> {
        let url = concat_string!(self.endpoint, "/orders?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_order<T: AsRef<str>>(&self, order_id: T) -> anyhow::Result<OrderResponseObject> {
        let url = concat_string!(self.endpoint, "/orders/", order_id);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_fills<T: AsRef<str>>(&self, address: T, subaccount_number: T, market: T) -> anyhow::Result<FillResponse> {
        let url = concat_string!(self.endpoint, "/fills?address=", address, "&subaccountNumber=", subaccount_number, "&market=", market, "&marketType=PERPETUAL&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_historical_pnl<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<HistoricalPnlResponse> {
        let url = concat_string!(self.endpoint, "/historical-pnl?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_transfers<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<TransferResponse> {
        let url = concat_string!(self.endpoint, "/transfers?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    // Get
    pub async fn get(&self, request: String) -> anyhow::Result<String> {
        let client = Client::new();

        println!("{}", request);
        let response = client.get(request)
            .header("Accept", "appplication/json")
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
