use base64::prelude::*;
use prost::Message;
use cosmrs::bip32::ExtendedPrivateKey;
use reqwest::Client;
use iop_keyvault::{Bip39, Bip44, Network, Subtree};
use iop_keyvault::secp256k1::hyd::Mainnet;
use cosmrs::{
    crypto::secp256k1::SigningKey,
    tx::{SignerInfo, Fee, BodyBuilder, SignDoc},
    Coin,
    Any,
};

use std::sync::Arc;
use std::collections::HashMap;

use crate::{
    Account, Tx, Market, 
    protobuf::{
        Order,
        SubaccountId,
        OrderId,
        MsgPlaceOrder,
        order::{
            GoodTilOneof,
            TimeInForce,
            ConditionType,
            Side,
        },
    },
    constants::*,
};

pub struct Cosmos;

impl Account {

    pub fn from_b64_key(key: String) -> anyhow::Result<Self> {

        let sk_bytes = BASE64_STANDARD.decode(key)?;
        let signing_key = SigningKey::from_slice(&sk_bytes).expect("Failed to read signing key");
        let public_key = signing_key.public_key();
        let id = public_key.account_id("dydx").expect("Failed to derive account ID");
        Ok(Self {
            signing_key: Arc::new(signing_key),
            public_key,
            id: id.to_string(),
        })
    }

    pub fn from_mnemonic(phrase: &str) -> anyhow::Result<Self> {
        let mnemonic = Bip39::new();
        let mnemonic = mnemonic.phrase(phrase)?;

        let seed = mnemonic.password("");
        println!("{:?}", seed);
        
        let n = Bip44::network(&Bip44, &seed, &Mainnet)?;
        println!("{:?}", n.to_xprv());
        
        // let signing_key = SigningKey::from(xpk);
        let signing_key = SigningKey::from_slice(&seed.as_bytes()).unwrap();
        let public_key = signing_key.public_key();
        let id = public_key.account_id("dydx").expect("Failed to derive account ID");
        println!("{:?}", id);
        Ok(Self {
            signing_key: Arc::new(signing_key),
            public_key,
            id: id.to_string(),
        })
    }

    pub async fn place_short_term_order(
        &self,
        subaccount_number: u32,
        client_id: u32,
        market: Market,
        side: Side, 
        size: f32,
        good_til_oneof: GoodTilOneof,
        time_in_force: TimeInForce,
        reduce_only: bool,
        client_metadata: u32,
        condition_type: ConditionType,
        trigger_price: f32,
    ) -> anyhow::Result<String> {
        
        let subaccount_id = SubaccountId {
            owner: self.id(),
            number: subaccount_number,
        };

        let order_id = OrderId {
            subaccount_id: Some(subaccount_id),
            client_id,
            order_flags: 0,
            clob_pair_id: market.id(),
        };

        let order = Order {
            order_id: Some(order_id),
            side: side.into(),
            quantums: 10000000,
            subticks: 3000000000,
            time_in_force: time_in_force.into(),
            reduce_only,
            client_metadata,
            condition_type: condition_type.into(),
            conditional_order_trigger_subticks: 0,
            good_til_oneof: Some(good_til_oneof),
        };

        let msg = MsgPlaceOrder {
            order: Some(order),
        };

        let any = Any {
            type_url: "/dydxprotocol.clob.MsgPlaceOrder".to_string(),
            value: msg.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(self.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            16933
        ).unwrap();

        let raw = sign_doc.sign(&self.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        /*
        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
        */
        Ok(String::new())

    }

    pub async fn place_stateful_order(
        &self
    ) -> anyhow::Result<String> {
        
        unimplemented!();
    }

    pub async fn cancel_short_term_order(
        &self
    ) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub async fn cancel_stateful_order(
        &self
    ) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub async fn transfer(
        &self,
    ) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub async fn deposit(
        &self,
    ) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub async fn withdraw(
        &self,
    ) -> anyhow::Result<String> {
        unimplemented!();
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

/*
impl Network for Cosmos {

    fn p2pkh_addr(&self) -> &'static [u8; 1] {
        [0; 1]
    }

    fn p2sh_addr(&self) -> &'static [u8; 1] {
        [0; 1]
    }

    fn wif(&self) -> &'static [u8; 1] {
        [0; 1]
    }

    fn bip32_xprv(&self) -> &'static [u8; 4] {
        [0; 4]
    }

    fn bip32_xpub(&self) -> &'static [u8; 4] {
        [0; 4] 
    }

    fn message_prefix(&self) -> &'static str {
        ""
    }

    fn slip44(&self) -> i32 {
        0
    }

    fn subtree(&self) -> &dyn Subtree<Suite = <Self as Subtree>::Suite> {
        Mainnet
    }
}
*/
