use crate::client::FaucetClient;
use crate::constants::*;

use reqwest::Client;
use concat_string::concat_string;

use std::collections::HashMap;

impl FaucetClient {

    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn fill(
        &self,
        address: &str,
        subaccount_number: u32,
        amount: u64,
    ) -> anyhow::Result<String> {
        let mut map = HashMap::new();
        map.insert("address", address.to_string());
        map.insert("subaccountNumber", subaccount_number.to_string());
        map.insert("amount", amount.to_string());

        self.post("/faucet/tokens", map).await
    }

    pub async fn fill_native(
        &self,
        address: &str,
    ) -> anyhow::Result<String> {
        let mut map = HashMap::new();
        map.insert("address", address.to_string());

        self.post("/faucet/native-token", map).await
    }

    async fn post(&self, path: &str, json: HashMap<&str, String>) -> anyhow::Result<String> {
        let client = Client::new();

        let url = concat_string!(self.endpoint, path);
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)

    }
}

impl Default for FaucetClient {

    fn default() -> Self {
        Self {
            endpoint: T_REST.to_string(),
        }
    }
}

