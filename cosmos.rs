// We allow non snake case so the transaction can be deserialized properly on the validator end.
#![allow(non_snake_case)]

use base64::prelude::*;
use prost::Message;
use reqwest::Client;
use cosmrs::{
    Any, Coin,
    tx::{
        SignerInfo,
        BodyBuilder,
        Fee,
        SignDoc,
    },
};

use std::collections::HashMap;

use crate::{
    constants::*,
    Subaccount,
    chain::message::{
        MsgPlaceOrder,
        MsgCancelOrder,
        MsgCreateTransfer,
        MsgWithdrawFromSubaccount,
        MsgDepositToSubaccount,
    },
};

pub struct Msg(Any);

// TODO: Remove hardcoded mainnet values
// Also make a trait so the code is less repetitive

impl MsgPlaceOrder {

    pub(crate) async fn execute(&self, subaccount: &Subaccount) -> anyhow::Result<String> {

        let any = Any {
            type_url: "/dydxprotocol.clob.MsgPlaceOrder".to_string(),
            value: self.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(subaccount.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            subaccount.account_id(),
        ).unwrap();

        let raw = sign_doc.sign(&subaccount.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

}

impl MsgCancelOrder {
    
    pub(crate) async fn execute(&self, subaccount: &Subaccount) -> anyhow::Result<String> {

        let any = Any {
            type_url: "/dydxprotocol.clob.MsgCancelOrder".to_string(),
            value: self.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(subaccount.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            subaccount.account_id(),
        ).unwrap();

        let raw = sign_doc.sign(&subaccount.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)

    }
}

impl MsgCreateTransfer {

    pub(crate) async fn execute(&self, subaccount: &Subaccount) -> anyhow::Result<String> {

        let any = Any {
            type_url: "/dydxprotocol.sending.MsgCreateTransfer".to_string(),
            value: self.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(subaccount.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            subaccount.account_id(),
        ).unwrap();

        let raw = sign_doc.sign(&subaccount.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)

    }

}

impl MsgDepositToSubaccount {

    pub(crate) async fn execute(&self, subaccount: &Subaccount) -> anyhow::Result<String> {

        let any = Any {
            type_url: "/dydxprotocol.sending.MsgDepositToSubaccount".to_string(),
            value: self.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(subaccount.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            subaccount.account_id(),
        ).unwrap();

        let raw = sign_doc.sign(&subaccount.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)

    }

}

impl MsgWithdrawFromSubaccount {

    pub(crate) async fn execute(&self, subaccount: &Subaccount) -> anyhow::Result<String> {

        let any = Any {
            type_url: "/dydxprotocol.sending.MsgWithdrawFromSubaccount".to_string(),
            value: self.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(subaccount.public_key()), 4);

        let coin = Coin::new(0, "adydx").unwrap();
        let fee = Fee::from_amount_and_gas(coin, 0u64);

        let auth_info = signer_info.auth_info(fee);
        let chain_id = "dydx-mainnet-1".parse()?;

        let sign_doc = SignDoc::new(
            &body,
            &auth_info,
            &chain_id,
            subaccount.account_id(),
        ).unwrap();

        let raw = sign_doc.sign(&subaccount.signing_key()).unwrap();
        let bytes = raw.to_bytes().unwrap();

        let txBytes = BASE64_STANDARD.encode(bytes);
        let mode = "BROADCAST_MODE_SYNC".to_string();

        let mut json = HashMap::new();
        json.insert("txBytes", txBytes);
        json.insert("mode", mode);

        let client = Client::new();

        let url = url::Url::parse(M_TX_ENDPOINT).unwrap();
        let response = client.post(url)
            .json(&json)
            .send()
            .await?
            .text()
            .await?;

        Ok(response)

    }
}
