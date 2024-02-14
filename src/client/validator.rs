use crate::{
    client::{
        ValidatorClient,
        Endpoints,
    },
    Market, Subaccount, 
    chain::message::{
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
};

impl ValidatorClient {

    pub fn new(subaccount: Subaccount, endpoints: Endpoints) -> Self {
        Self {
            subaccount,
            endpoints,
        }
    }

    pub async fn place_order(
        &self,
        subaccount_number: u32,
        client_id: u32,
        market: Market,
        side: Side,
        quantums: u64,
        subticks: u64,
        time_in_force: TimeInForce,
        order_flags: u8,
        reduce_only: bool,
        good_til_oneof: GoodTilOneof,
        client_metadata: u32,
        condition_type: ConditionType,
        conditional_order_trigger_subticks: u64,
    ) -> anyhow::Result<String> {

        let subaccount_id = SubaccountId {
            owner: self.subaccount.id(),
            number: subaccount_number,
        };

        let order_id = OrderId {
            subaccount_id: Some(subaccount_id),
            client_id,
            order_flags: order_flags as u32,
            clob_pair_id: market as u32,
        };

        let order = Order {
            order_id: Some(order_id),
            side: side.into(),
            quantums,
            subticks,
            time_in_force: time_in_force as i32,
            reduce_only,
            client_metadata,
            condition_type: condition_type as i32,
            conditional_order_trigger_subticks,
            good_til_oneof: Some(good_til_oneof),
        };

        let msg = MsgPlaceOrder {
            order: Some(order),
        };

        msg.execute(&self.subaccount, 16933).await
        /*

        let any = Any {
            type_url: "/dydxprotocol.clob.MsgPlaceOrder".to_string(),
            value: msg.encode_to_vec(),
        };

        let mut bodybuilder = BodyBuilder::new();
        bodybuilder.msg(any);
        let body = bodybuilder.finish();

        let signer_info = SignerInfo::single_direct(Some(self.subaccount.public_key()), 4);

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

        let raw = sign_doc.sign(&self.subaccount.signing_key()).unwrap();
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
        Ok(String::new())
        */

    }

    pub fn transfer(&self) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub fn withdraw(&self) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub fn deposit_to_subaccount(&self) -> anyhow::Result<String> {
        unimplemented!();
    }

    pub fn withdraw_from_subaccount(&self) -> anyhow::Result<String> {
        unimplemented!();
    }

}
