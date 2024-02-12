use cosmrs::crypto;

use std::sync::Arc;

pub(crate) mod market_mappings;
pub(crate) mod cosmos;
pub(crate) mod account;
pub(crate) mod constants;
pub(crate) mod protobuf;
pub(crate) mod indexer;
pub(crate) mod client;
pub(crate) mod faucet;
pub(crate) mod transfers;
pub(crate) mod websocket;

pub struct Account {
    signing_key: Arc<crypto::secp256k1::SigningKey>,
    public_key: crypto::PublicKey,
    id: String,
}

pub struct Tx {

}

#[derive(Debug)]
pub enum Market {
    BTC,
    ETH,
    SOL,
    FIL,
    AVAX,
    MATIC,
    NEAR,
    CRV,
    LINK,
    SNX,
    DOT,
    DOGE,
    ATOM,
    ENJ,
    AAVE,
    COMP,
    ALGO,
    ZRX,
    EOS,
    ICP,
    TRX,
    XTZ,
    INCH,
    SUSHI,
    CELO,
    UNI,
    ETC,
}

#[cfg(test)]
mod test {

    use super::*;
    use std::env;

    use crate::{
        Market,
        protobuf::{
            order::{
                Side,
                GoodTilOneof,
                TimeInForce,
                ConditionType,
            },
        },
    };

    #[tokio::test]
    pub async fn runthrough() {
        dotenv::dotenv().ok();

        let mut private_key: String = env::var("MNEMONIC").expect("No private key found in environment");

        let account = Account::from_mnemonic(&private_key).unwrap();
        println!("{:?}", account.id);

        let response = account.place_short_term_order(
            0,
            33,
            Market::ETH,
            Side::Buy,
            0.01,
            GoodTilOneof::GoodTilBlock(200),
            TimeInForce::Ioc,
            false,
            0,
            ConditionType::Unspecified,
            3000.0,
        ).await.unwrap();
        println!("{:?}", response);
    }
}
