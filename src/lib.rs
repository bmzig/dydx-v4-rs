use cosmrs::{
    crypto::{self, secp256k1::SigningKey},
    bip32::{Mnemonic, Language},
};
use base64::prelude::*;
use reqwest::Client;
use concat_string::concat_string;

use crate::constants::*;

use std::sync::Arc;

pub mod client;
pub mod chain;
pub mod constants;

pub struct Subaccount {
    signing_key: Arc<SigningKey>,
    public_key: crypto::PublicKey,
    id: String,
    number: u32, // This is the subaccount number
    account_number: u64, // This is the cosmos assigned account number used for ordering
}

#[derive(Debug)]
#[repr(u32)]
pub enum Market {
    BTC = 0,
    ETH = 1,
    SOL = 5,
    AVAX = 7,
    MATIC = 3,
    NEAR = 16,
    LINK = 2,
    ATOM = 11,
    TRX = 15,
    UNI = 13,
    ETC = 19,
    APE = 22,
    PEPE = 28,
    SEI = 29,
    LTC = 9,
    CRV = 4,
    ARB = 24,
    SUI = 31,
    AAVE = 36,
    SHIB = 30,
    DOGE = 10,
    MKR = 17,
    WLD = 21,
    BLUR = 25,
    ADA = 6,
    APT = 23,
    FIL = 8,
    COMP = 20,
    TIA = 33,
    JUP = 35,
    DOT = 12,
    LDO = 26,
    BNB = 37,
    XRP = 32,
    OP = 27,
    BCH = 14,
    XLM = 18,
}

pub enum OrderType {
    Market,
    Limit,
    StopMarket,
    StopLimit,
    TakeProfitMarket,
    TakeProfitLimit,
}

impl Subaccount {

    // TODO: Differentiate between testnet and mainnet
    // TODO: Consider getting the account number in a different manner. 
    pub async fn from_b64_key(key: String) -> anyhow::Result<Self> {

        let sk_bytes = BASE64_STANDARD.decode(key)?;
        let signing_key = SigningKey::from_slice(&sk_bytes).expect("Failed to read signing key");
        let public_key = signing_key.public_key();
        let id = public_key.account_id("dydx").expect("Failed to derive account ID");

        let url = concat_string!(M_ACC_ENDPOINT, "/", id);
        let client = Client::new();

        let response = client.get(url)
            .header("Accept", "application/json")
            .send()
            .await?
            .text()
            .await?;

        let split = response.split(",").collect::<Vec<&str>>();
        let account_number = split[split.len()-2].split("\"").collect::<Vec<&str>>()[3].parse::<u64>().unwrap();

        Ok(Self {
            signing_key: Arc::new(signing_key),
            public_key,
            id: id.to_string(),
            number: 0,
            account_number,
        })
    }

    pub async fn from_mnemonic(phrase: &str) -> anyhow::Result<Self> {
        let mnemonic = Mnemonic::new(phrase, Language::default())?;
        let seed = mnemonic.to_seed("");
        
        let child_xprv = cosmrs::bip32::XPrv::derive_from_path(&seed.as_bytes(), &CHILD_PATH.parse().unwrap()).unwrap();
        
        let signing_key = SigningKey::from(child_xprv);
        let public_key = signing_key.public_key();
        let id = public_key.account_id("dydx").expect("Failed to derive account ID");

        let url = concat_string!(M_ACC_ENDPOINT, "/", id);
        let client = Client::new();

        let response = client.get(url)
            .header("Accept", "application/json")
            .send()
            .await?
            .text()
            .await?;
        
        let split = response.split(",").collect::<Vec<&str>>();
        let account_number = split[split.len()-2].split("\"").collect::<Vec<&str>>()[3].parse::<u64>().unwrap();

        Ok(Self {
            signing_key: Arc::new(signing_key),
            public_key,
            id: id.to_string(),
            number: 0,
            account_number,
        })
    }

    pub(crate) fn account_number(&self) -> u64 {
        self.account_number
    }

    pub(crate) fn number(&self) -> u32 {
        self.number
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    // TODO: Consider using something else?
    pub fn signing_key(&self) -> Arc<SigningKey> {
        Arc::clone(&self.signing_key)
    }

    // TODO: Consider using an Arc/Rc
    pub fn public_key(&self) -> cosmrs::crypto::PublicKey {
        self.public_key
    }

}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    use crate::{
        Market,
        chain::message::order::{Side, TimeInForce},
        client::{CompositeClient, Endpoints},
    };

    #[tokio::test]
    pub async fn runthrough() {
        dotenv::dotenv().ok();

        let private_key: String = env::var("MNEMONIC").expect("No private key found in environment");
        // let b64: String = env::var("PRIVATE_KEY").unwrap();

        let account = Subaccount::from_mnemonic(&private_key).await.unwrap();
        let id = account.id();
        let composite = CompositeClient::new(account, Endpoints::mainnet(), M_IAPI);

        let acc = composite.indexer.get_address(id).await.unwrap();
        println!("{:?}", acc);

        let response = composite.indexer.list_perpetual_markets().await.unwrap();
        // println!("{:?}", response);

        let v = Market::vec();
        for market in v {
            // println!("{}", market);
            assert!(market.atomic_resolution() == response.markets[&market.to_string()].atomicResolution as i8);
            assert!(market.tick_size() == response.markets[&market.to_string()].tickSize.parse::<f32>().unwrap());
            assert!(market.step_size() == response.markets[&market.to_string()].stepSize.parse::<f32>().unwrap());
            assert!(market.subticks_per_tick() == response.markets[&market.to_string()].subticksPerTick as u64);
        }

        let order_response = composite.place_short_term_order(
            Market::ETH,
            Side::Buy,
            3000.0,
            0.01,
            33,
            200,
            TimeInForce::Ioc,
            false,
            None,
        ).await.unwrap();
        println!("{}", order_response);
        let cancel_response = composite.cancel_short_term_order(
            23,
            Market::ETH,
            200,
        ).await.unwrap();
        println!("{}", cancel_response);
    }
}
