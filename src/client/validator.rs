use crate::{
    client::{
        ValidatorClient,
        Endpoints,
    },
    Market, Subaccount, 
    chain::{
        message::{
            Order,
            SubaccountId,
            OrderId,
            MsgPlaceOrder,
            MsgCancelOrder,
            order::{
                GoodTilOneof,
                TimeInForce,
                ConditionType,
                Side,
            },
            msg_cancel_order,
            MsgCreateTransfer,
            Transfer,
            MsgDepositToSubaccount,
            MsgWithdrawFromSubaccount,
            Coin,
        },
        cosmos::Msg,
    },
};

use cosmrs::Any;
use prost::Message;

impl ValidatorClient {

    pub fn new(endpoints: Endpoints) -> Self {
        Self {
            endpoints,
        }
    }

    pub async fn place_order(
        &self,
        subaccount: &Subaccount,
        client_id: u32,
        market: Market,
        side: Side,
        quantums: u64,
        subticks: u64,
        time_in_force: TimeInForce,
        order_flags: u32,
        reduce_only: bool,
        good_til_oneof: GoodTilOneof,
        client_metadata: u32,
        condition_type: ConditionType,
        conditional_order_trigger_subticks: u64,
    ) -> anyhow::Result<String> {

        let subaccount_id = SubaccountId {
            owner: subaccount.id(),
            number: subaccount.number(),
        };

        let order_id = OrderId {
            subaccount_id: Some(subaccount_id),
            client_id,
            order_flags: order_flags,
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

        let msg_place_order = MsgPlaceOrder {
            order: Some(order),
        };

        let msg = Msg {
            any: Any {
                type_url: "/dydxprotocol.clob.MsgPlaceOrder".to_string(),
                value: msg_place_order.encode_to_vec(),
            },
        };

        msg.execute(&subaccount).await
        
    }
    
    pub async fn cancel_order(
        &self,
        subaccount: &Subaccount,
        client_id: u32,
        clob_pair_id: u32,
        order_flags: u32,
        good_til_oneof: GoodTilOneof,
    ) -> anyhow::Result<String> {

        // TODO: Clean this up probably by tweaking protobuf messages
        let good_til_oneof = {
            match good_til_oneof {
                GoodTilOneof::GoodTilBlock(x) => msg_cancel_order::GoodTilOneof::GoodTilBlock(x),
                GoodTilOneof::GoodTilBlockTime(x) => msg_cancel_order::GoodTilOneof::GoodTilBlockTime(x),
            }
        };
        
        let subaccount_id = SubaccountId {
            owner: subaccount.id(),
            number: subaccount.number(),
        };

        let order_id = OrderId {
            subaccount_id: Some(subaccount_id),
            client_id,
            order_flags,
            clob_pair_id,
        };

        let msg_cancel_order = MsgCancelOrder {
            order_id: Some(order_id),
            good_til_oneof: Some(good_til_oneof),
        };

        let msg = Msg {
            any: Any {
                type_url: "/dydxprotocol.clob.MsgCancelOrder".to_string(),
                value: msg_cancel_order.encode_to_vec(),
            },
        };


        msg.execute(subaccount).await
    }

    pub async fn transfer(
        &self,
        subaccount: &Subaccount,
        recipient_address: String,
        subaccount_number: u32,
        asset_id: u32,
        amount: u64,
    ) -> anyhow::Result<String> {
        let sender = SubaccountId {
            owner: subaccount.id(),
            number: subaccount.number(),
        };

        let recipient = SubaccountId {
            owner: recipient_address,
            number: subaccount_number,
        };

        let transfer = Transfer {
            sender: Some(sender),
            recipient: Some(recipient),
            asset_id,
            amount,
        };

        let msg_transfer = MsgCreateTransfer {
            transfer: Some(transfer),
        };

        let msg = Msg {
            any: Any {
                type_url: "/dydxprotocol.sending.MsgCreateTransfer".to_string(),
                value: msg_transfer.encode_to_vec(),
            },
        };

        msg.execute(subaccount).await
    }

    pub async fn deposit(
        &self,
        subaccount: &Subaccount,
        asset_id: u32,
        quantums: u64,
    ) -> anyhow::Result<String> {

        let recipient = SubaccountId {
            owner: subaccount.id(),
            number: subaccount.number(),
        };

        let msg_deposit = MsgDepositToSubaccount {
            sender: subaccount.id(),
            recipient: Some(recipient),
            asset_id,
            quantums,
        };

        let msg = Msg {
            any: Any {
                type_url: "/dydxprotocol.sending.MsgDepositToSubaccount".to_string(),
                value: msg_deposit.encode_to_vec(),
            },
        };

        msg.execute(subaccount).await
    }

    pub async fn withdraw(
        &self,
        subaccount: &Subaccount,
        asset_id: u32,
        quantums: u64,
    ) -> anyhow::Result<String> {

        let sender = SubaccountId {
            owner: subaccount.id(),
            number: subaccount.number(),
        };

        let msg_withdraw = MsgWithdrawFromSubaccount {
            sender: Some(sender),
            recipient: subaccount.id(),
            asset_id,
            quantums,
        };

        let msg = Msg {
            any: Any {
                type_url: "/dydxprotocol.sending.MsgWithdrawFromSubaccount".to_string(),
                value: msg_withdraw.encode_to_vec(),
            },
        };

        msg.execute(subaccount).await

    }

}
