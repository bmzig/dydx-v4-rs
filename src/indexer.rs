use reqwest::{Url, Client};
use concat_string::concat_string;
use serde_json::Value;

use crate::{
    client::IndexerClient,
};

#[macro_use(concat_string)]

impl IndexerClient {

    // API
    // Indexer Status

    pub async fn get_height(&self) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/height/");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    // Markets

    pub async fn list_perpetual_markets(&self) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/perpetualMarkets?limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    pub async fn get_perpetual_market<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/orderbooks/perpetualMarket/", ticker);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn list_sparklines(&self) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/sparklines?timePeriod=SEVEN_DAYS");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_orderbook<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/orderbooks/perpetualMarket/", ticker);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_trades<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/trades/perpetualMarket/", ticker, "?limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_historical_funding<T: AsRef<str>>(&self, ticker: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/historicalFunding/", ticker, "?limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_candles<T: AsRef<str>>(&self, ticker: T, resolution: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/candles/perpetualMarkets/", ticker, "?resolution=", resolution, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    // Subaccounts

    pub async fn get_address<T: AsRef<str>>(&self, address: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/addresses/", address);
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }
    
    pub async fn get_subaccount<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/addresses/", address, "/subaccountNumber/", subaccount_number);
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    pub async fn get_asset_positions<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/assetPositions?address=", address, "&subaccountNumber=", subaccount_number);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_perpetual_positions<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/perpetualPositions?address=", address, "&subaccountNumber=", subaccount_number, "&status=OPEN&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_orders<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/orders?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_order<T: AsRef<str>>(&self, order_id: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/orders/", order_id);
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_fills<T: AsRef<str>>(&self, address: T, subaccount_number: T, market: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/fills?address=", address, "&subaccountNumber=", subaccount_number, "&market=", market, "&marketType=PERPETUAL&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_historical_pnl<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/historical-pnl?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
        
    }

    pub async fn get_transfers<T: AsRef<str>>(&self, address: T, subaccount_number: T) -> anyhow::Result<Value> {
        let url = concat_string!(self.rest(), "v4/transfers?address=", address, "&subaccountNumber=", subaccount_number, "&limit=0");
        Ok(serde_json::from_str(&self.get(url).await?)?)
    }

    // Get
    pub async fn get(&self, request: String) -> anyhow::Result<String> {
        let client = Client::new();

        let response = client.get(request)
            .header("Accept", "appplication/json")
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }
}
