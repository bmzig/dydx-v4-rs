use crate::{
    client::{
        CompositeClient,
        ValidatorClient,
        IndexerClient,
        Endpoints,
    },
    Subaccount, Market, OrderType,
    chain::message::{
        order::{
            GoodTilOneof,
            TimeInForce,
            ConditionType,
            Side,
        },
    },
    constants::*,
};

// TODO: The subaccount_number flag should be moved to a field in the Subaccount struct.

impl CompositeClient {

    pub fn new(validator_endpoints: Endpoints, indexer_endpoint: &str) -> Self {
        let indexer = IndexerClient::new(indexer_endpoint);
        let validator = ValidatorClient::new(validator_endpoints);
        Self {
            indexer,
            validator,
        }
    }

    pub async fn place_short_term_order(
        &self,
        subaccount: &Subaccount,
        market: Market,
        side: Side, 
        price: f32,
        size: f32,
        client_id: u32,
        good_til_block: u32,
        time_in_force: TimeInForce,
        reduce_only: bool,
        client_metadata: Option<u32>,
    ) -> anyhow::Result<String> {
       
        let good_til_oneof = GoodTilOneof::GoodTilBlock(good_til_block);

        let raw_quantums = (size * 10u64.pow((-1 * market.atomic_resolution()) as u32) as f32) as u64;
        let mut quantums = (raw_quantums/STEP_BASE_QUANTUMS) * STEP_BASE_QUANTUMS;
        if STEP_BASE_QUANTUMS > quantums { quantums = STEP_BASE_QUANTUMS; }

        let exponent = (market.atomic_resolution() - QUANTUM_CONVERSION_EXPONENT - QUOTE_QUANTUMS_ATOMIC_RESOLUTION) as u32;
        let raw_subticks = (price * 10u64.pow(exponent) as f32) as u64;
        let mut subticks = (raw_subticks/market.subticks_per_tick()) * market.subticks_per_tick();
        if market.subticks_per_tick() > subticks { subticks = market.subticks_per_tick(); }

        self.validator.place_order(
            subaccount,
            client_id,
            market,
            side,
            quantums,
            subticks,
            time_in_force,
            ORDER_FLAGS_SHORT_TERM,
            reduce_only,
            good_til_oneof,
            client_metadata.unwrap_or(0),
            ConditionType::Unspecified,
            0,
        ).await

    }

    pub async fn place_order(
        &self,
        subaccount: &Subaccount,
        market: Market,
        order_type: OrderType,
        side: Side,
        price: f32,
        size: f32,
        client_id: u32,
        time_in_force: TimeInForce,
        good_til_oneof: GoodTilOneof,
        reduce_only: bool,
        post_only: bool,
        trigger_price: Option<f32>,
        client_metadata: Option<u32>,
    ) -> anyhow::Result<String> {
        
        let raw_quantums = (size * 10u64.pow((-1 * market.atomic_resolution()) as u32) as f32) as u64;
        let mut quantums = (raw_quantums/STEP_BASE_QUANTUMS) * STEP_BASE_QUANTUMS;
        if STEP_BASE_QUANTUMS > quantums { quantums = STEP_BASE_QUANTUMS; }

        let exponent = (market.atomic_resolution() - QUANTUM_CONVERSION_EXPONENT - QUOTE_QUANTUMS_ATOMIC_RESOLUTION) as u32;
        let raw_subticks = (price * 10u64.pow(exponent) as f32) as u64;
        let mut subticks = (raw_subticks/market.subticks_per_tick()) * market.subticks_per_tick();
        if market.subticks_per_tick() > subticks { subticks = market.subticks_per_tick(); }

        let mut conditional_order_trigger_subticks = None;
        if let Some(tp) = trigger_price {

            let raw_trigger_subticks = (tp * 10u64.pow(exponent) as f32) as u64;
            let mut trigger_subticks = (raw_trigger_subticks/market.subticks_per_tick()) * market.subticks_per_tick();
            if market.subticks_per_tick() > trigger_subticks { trigger_subticks = market.subticks_per_tick(); }

            conditional_order_trigger_subticks = Some(trigger_subticks);
        }

        let mut order_flags = ORDER_FLAGS_CONDITIONAL;
        let mut condition_type = ConditionType::Unspecified;
        match order_type {
            OrderType::Market => order_flags = ORDER_FLAGS_SHORT_TERM,
            OrderType::Limit => {
                if post_only { order_flags = ORDER_FLAGS_SHORT_TERM; }
                else { order_flags = ORDER_FLAGS_LONG_TERM; }
            },
            OrderType::StopMarket | OrderType::StopLimit => condition_type = ConditionType::StopLoss,
            _ => condition_type = ConditionType::TakeProfit,
        };

        self.validator.place_order(
            subaccount,
            client_id,
            market,
            side,
            quantums,
            subticks,
            time_in_force,
            order_flags,
            reduce_only,
            good_til_oneof,
            client_metadata.unwrap_or(0),
            condition_type,
            conditional_order_trigger_subticks.unwrap_or(0),
        ).await
    }

    pub async fn cancel_short_term_order(
        &self,
        subaccount: &Subaccount,
        client_id: u32,
        market: Market,
        good_til_block: u32,
    ) -> anyhow::Result<String> {

    let good_til_oneof = GoodTilOneof::GoodTilBlock(good_til_block);
        let orderbook_id = market as u32;

        self.validator.cancel_order(
            subaccount,
            client_id, 
            orderbook_id, 
            ORDER_FLAGS_SHORT_TERM, 
            good_til_oneof
            ).await
    }

    pub async fn cancel_order(
        &self,
        subaccount: &Subaccount,
        client_id: u32,
        market: Market,
        order_flags: u32,
        good_til_oneof: GoodTilOneof,
    ) -> anyhow::Result<String> {

        let orderbook_id = market as u32;

        self.validator.cancel_order(
            subaccount,
            client_id, 
            orderbook_id, 
            order_flags, 
            good_til_oneof
        ).await
    }

    pub async fn transfer(
        &self,
        subaccount: &Subaccount,
        recipient_address: String,
        recipient_subaccount_number: u32,
        amount: f32,
    ) -> anyhow::Result<String> {
        let int_amount = ((amount * 10000000.0).floor()) as u64;
        self.validator.transfer(
            subaccount,
            recipient_address, 
            recipient_subaccount_number, 
            0, 
            int_amount
        ).await
    }

    pub async fn deposit_to_subaccount(
        &self,
        subaccount: &Subaccount,
        amount: f32,
    ) -> anyhow::Result<String> {
        let int_amount = ((amount * 10000000.0).floor()) as u64;
        self.validator.deposit(
            subaccount,
            0,
            int_amount
        ).await
    }

    pub async fn withdraw_from_subaccount(
        &self,
        subaccount: &Subaccount,
        amount: f32,
    ) -> anyhow::Result<String> {

        let quantums = ((amount * 10000000.0).floor()) as u64;
        self.validator.withdraw(
            subaccount,
            0,
            quantums
        ).await
    }

}

