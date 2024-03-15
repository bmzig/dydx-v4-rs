#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPlaceOrder { 
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// AssetPositions define an account’s positions of an `Asset`.
/// Therefore they hold any information needed to trade on Spot and Margin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPosition {
    /// The `Id` of the `Asset`.
    #[prost(uint32, tag = "1")]
    pub asset_id: u32,
    /// The absolute size of the position in base quantums.
    #[prost(bytes = "vec", tag = "2")]
    pub quantums: ::prost::alloc::vec::Vec<u8>,
    /// The `Index` (either `LongIndex` or `ShortIndex`) of the `Asset` the last
    /// time this position was settled
    /// TODO(DEC-582): pending margin trading being added.
    #[prost(uint64, tag = "3")]
    pub index: u64,
}
/// PerpetualPositions are an account’s positions of a `Perpetual`.
/// Therefore they hold any information needed to trade perpetuals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerpetualPosition {
    /// The `Id` of the `Perpetual`.
    #[prost(uint32, tag = "1")]
    pub perpetual_id: u32,
    /// The size of the position in base quantums.
    #[prost(bytes = "vec", tag = "2")]
    pub quantums: ::prost::alloc::vec::Vec<u8>,
    /// The funding_index of the `Perpetual` the last time this position was
    /// settled.
    #[prost(bytes = "vec", tag = "3")]
    pub funding_index: ::prost::alloc::vec::Vec<u8>,
}
/// SubaccountId defines a unique identifier for a Subaccount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubaccountId {
    /// The address of the wallet that owns this subaccount.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// < 128 Since 128 should be enough to start and it fits within
    /// 1 Byte (1 Bit needed to indicate that the first byte is the last).
    #[prost(uint32, tag = "2")]
    pub number: u32,
}
/// Subaccount defines a single sub-account for a given address.
/// Subaccounts are uniquely indexed by a subaccountNumber/owner pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subaccount {
    /// The Id of the Subaccount
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<SubaccountId>,
    /// All `AssetPosition`s associated with this subaccount.
    /// Always sorted ascending by `asset_id`.
    #[prost(message, repeated, tag = "2")]
    pub asset_positions: ::prost::alloc::vec::Vec<AssetPosition>,
    /// All `PerpetualPosition`s associated with this subaccount.
    /// Always sorted ascending by `perpetual_id.
    #[prost(message, repeated, tag = "3")]
    pub perpetual_positions: ::prost::alloc::vec::Vec<PerpetualPosition>,
    /// Set by the owner. If true, then margin trades can be made in this
    /// subaccount.
    #[prost(bool, tag = "4")]
    pub margin_enabled: bool,
}
/// OrderId refers to a single order belonging to a Subaccount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderId {
    /// The subaccount ID that opened this order.
    /// Note that this field has `gogoproto.nullable = false` so that it is
    /// generated as a value instead of a pointer. This is because the `OrderId`
    /// proto is used as a key within maps, and map comparisons will compare
    /// pointers for equality (when the desired behavior is to compare the values).
    #[prost(message, optional, tag = "1")]
    pub subaccount_id: ::core::option::Option<SubaccountId>,
    /// The client ID of this order, unique with respect to the specific
    /// sub account (I.E., the same subaccount can't have two orders with
    /// the same ClientId).
    #[prost(fixed32, tag = "2")]
    pub client_id: u32,
    /// order_flags represent order flags for the order. This field is invalid if
    /// it's greater than 127 (larger than one byte). Each bit in the first byte
    /// represents a different flag. Currently only two flags are supported.
    ///
    /// Starting from the bit after the most MSB (note that the MSB is used in
    /// proto varint encoding, and therefore cannot be used): Bit 1 is set if this
    /// order is a Long-Term order (0x40, or 64 as a uint8). Bit 2 is set if this
    /// order is a Conditional order (0x20, or 32 as a uint8).
    ///
    /// If neither bit is set, the order is assumed to be a Short-Term order.
    ///
    /// If both bits are set or bits other than the 2nd and 3rd are set, the order
    /// ID is invalid.
    #[prost(uint32, tag = "3")]
    pub order_flags: u32,
    /// ID of the CLOB the order is created for.
    #[prost(uint32, tag = "4")]
    pub clob_pair_id: u32,
}
/// OrdersFilledDuringLatestBlock represents a list of `OrderIds` that were
/// filled by any non-zero amount in the latest block.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrdersFilledDuringLatestBlock {
    /// A list of unique order_ids that were filled by any non-zero amount in the
    /// latest block.
    #[prost(message, repeated, tag = "1")]
    pub order_ids: ::prost::alloc::vec::Vec<OrderId>,
}
/// PotentiallyPrunableOrders represents a list of orders that may be prunable
/// from state at a future block height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PotentiallyPrunableOrders {
    /// A list of unique order_ids that may potentially be pruned from state at a
    /// future block height.
    #[prost(message, repeated, tag = "1")]
    pub order_ids: ::prost::alloc::vec::Vec<OrderId>,
}
/// OrderFillState represents the fill amount of an order according to on-chain
/// state. This proto includes both the current on-chain fill amount of the
/// order, as well as the block at which this information can be pruned from
/// state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderFillState {
    /// The current fillAmount of the order according to on-chain state.
    #[prost(uint64, tag = "1")]
    pub fill_amount: u64,
    /// The block height at which the fillAmount state for this order can be
    /// pruned.
    #[prost(uint32, tag = "2")]
    pub prunable_block_height: u32,
}
/// StatefulOrderTimeSliceValue represents the type of the value of the
/// `StatefulOrdersTimeSlice` in state. The `StatefulOrdersTimeSlice`
/// in state consists of key/value pairs where the keys are UTF-8-encoded
/// `RFC3339NANO` timestamp strings with right-padded zeroes and no
/// time zone info, and the values are of type `StatefulOrderTimeSliceValue`.
/// This `StatefulOrderTimeSliceValue` in state is used for managing stateful
/// order expiration. Stateful order expirations can be for either long term
/// or conditional orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulOrderTimeSliceValue {
    /// A unique list of order_ids that expire at this timestamp, sorted in
    /// ascending order by block height and transaction index of each stateful
    /// order.
    #[prost(message, repeated, tag = "1")]
    pub order_ids: ::prost::alloc::vec::Vec<OrderId>,
}
/// LongTermOrderPlacement represents the placement of a stateful order in
/// state. It stores the stateful order itself and the `BlockHeight` and
/// `TransactionIndex` at which the order was placed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LongTermOrderPlacement {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
    /// The block height and transaction index at which the order was placed.
    /// Used for ordering by time priority when the chain is restarted.
    #[prost(message, optional, tag = "2")]
    pub placement_index: ::core::option::Option<TransactionOrdering>,
}
/// ConditionalOrderPlacement represents the placement of a conditional order in
/// state. It stores the stateful order itself, the `BlockHeight` and
/// `TransactionIndex` at which the order was placed and triggered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalOrderPlacement {
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
    /// The block height and transaction index at which the order was placed.
    #[prost(message, optional, tag = "2")]
    pub placement_index: ::core::option::Option<TransactionOrdering>,
    /// The block height and transaction index at which the order was triggered.
    /// Set to be nil if the transaction has not been triggered.
    /// Used for ordering by time priority when the chain is restarted.
    #[prost(message, optional, tag = "3")]
    pub trigger_index: ::core::option::Option<TransactionOrdering>,
}
/// Order represents a single order belonging to a `Subaccount`
/// for a particular `ClobPair`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// The unique ID of this order. Meant to be unique across all orders.
    #[prost(message, optional, tag = "1")]
    pub order_id: ::core::option::Option<OrderId>,
    #[prost(enumeration = "order::Side", tag = "2")]
    pub side: i32,
    /// The size of this order in base quantums. Must be a multiple of
    /// `ClobPair.StepBaseQuantums` (where `ClobPair.Id = orderId.ClobPairId`).
    #[prost(uint64, tag = "3")]
    pub quantums: u64,
    /// The price level that this order will be placed at on the orderbook,
    /// in subticks. Must be a multiple of ClobPair.SubticksPerTick
    /// (where `ClobPair.Id = orderId.ClobPairId`).
    #[prost(uint64, tag = "4")]
    pub subticks: u64,
    /// The time in force of this order.
    #[prost(enumeration = "order::TimeInForce", tag = "7")]
    pub time_in_force: i32,
    /// Enforces that the order can only reduce the size of an existing position.
    /// If a ReduceOnly order would change the side of the existing position,
    /// its size is reduced to that of the remaining size of the position.
    /// If existing orders on the book with ReduceOnly
    /// would already close the position, the least aggressive (out-of-the-money)
    /// ReduceOnly orders are resized and canceled first.
    #[prost(bool, tag = "8")]
    pub reduce_only: bool,
    /// Set of bit flags set arbitrarily by clients and ignored by the protocol.
    /// Used by indexer to infer information about a placed order.
    #[prost(uint32, tag = "9")]
    pub client_metadata: u32,
    #[prost(enumeration = "order::ConditionType", tag = "10")]
    pub condition_type: i32,
    /// conditional_order_trigger_subticks represents the price at which this order
    /// will be triggered. If the condition_type is CONDITION_TYPE_UNSPECIFIED,
    /// this value is enforced to be 0. If this value is nonzero, condition_type
    /// cannot be CONDITION_TYPE_UNSPECIFIED. Value is in subticks.
    /// Must be a multiple of ClobPair.SubticksPerTick (where `ClobPair.Id =
    /// orderId.ClobPairId`).
    #[prost(uint64, tag = "11")]
    pub conditional_order_trigger_subticks: u64,
    /// Information about when the order expires.
    #[prost(oneof = "order::GoodTilOneof", tags = "5, 6")]
    pub good_til_oneof: ::core::option::Option<order::GoodTilOneof>,
}
/// Nested message and enum types in `Order`.
pub mod order {
    /// Represents the side of the orderbook the order will be placed on.
    /// Note that Side.SIDE_UNSPECIFIED is an invalid order and cannot be
    /// placed on the orderbook.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Side {
        /// Default value. This value is invalid and unused.
        Unspecified = 0,
        /// SIDE_BUY is used to represent a BUY order.
        Buy = 1,
        /// SIDE_SELL is used to represent a SELL order.
        Sell = 2,
    }
    impl Side {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Side::Unspecified => "SIDE_UNSPECIFIED",
                Side::Buy => "SIDE_BUY",
                Side::Sell => "SIDE_SELL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "SIDE_UNSPECIFIED" => Some(Self::Unspecified),
                "SIDE_BUY" => Some(Self::Buy),
                "SIDE_SELL" => Some(Self::Sell),
                _ => None,
            }
        }
    }
    /// TimeInForce indicates how long an order will remain active before it
    /// is executed or expires.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum TimeInForce {
        /// TIME_IN_FORCE_UNSPECIFIED represents the default behavior where an
        /// order will first match with existing orders on the book, and any
        /// remaining size will be added to the book as a maker order.
        Unspecified = 0,
        /// TIME_IN_FORCE_IOC enforces that an order only be matched with
        /// maker orders on the book. If the order has remaining size after
        /// matching with existing orders on the book, the remaining size
        /// is not placed on the book.
        Ioc = 1,
        /// TIME_IN_FORCE_POST_ONLY enforces that an order only be placed
        /// on the book as a maker order. Note this means that validators will cancel
        /// any newly-placed post only orders that would cross with other maker
        /// orders.
        PostOnly = 2,
        /// TIME_IN_FORCE_FILL_OR_KILL enforces that an order will either be filled
        /// completely and immediately by maker orders on the book or canceled if the
        /// entire amount can‘t be matched.
        FillOrKill = 3,
    }
    impl TimeInForce {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeInForce::Unspecified => "TIME_IN_FORCE_UNSPECIFIED",
                TimeInForce::Ioc => "TIME_IN_FORCE_IOC",
                TimeInForce::PostOnly => "TIME_IN_FORCE_POST_ONLY",
                TimeInForce::FillOrKill => "TIME_IN_FORCE_FILL_OR_KILL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIME_IN_FORCE_UNSPECIFIED" => Some(Self::Unspecified),
                "TIME_IN_FORCE_IOC" => Some(Self::Ioc),
                "TIME_IN_FORCE_POST_ONLY" => Some(Self::PostOnly),
                "TIME_IN_FORCE_FILL_OR_KILL" => Some(Self::FillOrKill),
                _ => None,
            }
        }
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ConditionType {
        /// CONDITION_TYPE_UNSPECIFIED represents the default behavior where an
        /// order will be placed immediately on the orderbook.
        Unspecified = 0,
        /// CONDITION_TYPE_STOP_LOSS represents a stop order. A stop order will
        /// trigger when the oracle price moves at or above the trigger price for
        /// buys, and at or below the trigger price for sells.
        StopLoss = 1,
        /// CONDITION_TYPE_TAKE_PROFIT represents a take profit order. A take profit
        /// order will trigger when the oracle price moves at or below the trigger
        /// price for buys and at or above the trigger price for sells.
        TakeProfit = 2,
    }
    impl ConditionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConditionType::Unspecified => "CONDITION_TYPE_UNSPECIFIED",
                ConditionType::StopLoss => "CONDITION_TYPE_STOP_LOSS",
                ConditionType::TakeProfit => "CONDITION_TYPE_TAKE_PROFIT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CONDITION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "CONDITION_TYPE_STOP_LOSS" => Some(Self::StopLoss),
                "CONDITION_TYPE_TAKE_PROFIT" => Some(Self::TakeProfit),
                _ => None,
            }
        }
    }
    /// Information about when the order expires.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GoodTilOneof {
        /// The last block this order can be executed at (after which it will be
        /// unfillable). Used only for Short-Term orders. If this value is non-zero
        /// then the order is assumed to be a Short-Term order.
        #[prost(uint32, tag = "5")]
        GoodTilBlock(u32),
        /// good_til_block_time represents the unix timestamp (in seconds) at which a
        /// stateful order will be considered expired. The
        /// good_til_block_time is always evaluated against the previous block's
        /// `BlockTime` instead of the block in which the order is committed. If this
        /// value is non-zero then the order is assumed to be a stateful or
        /// conditional order.
        #[prost(fixed32, tag = "6")]
        GoodTilBlockTime(u32),
    }
}
/// TransactionOrdering represents a unique location in the block where a
/// transaction was placed. This proto includes both block height and the
/// transaction index that the specific transaction was placed. This information
/// is used for ordering by time priority when the chain is restarted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionOrdering {
    /// Block height in which the transaction was placed.
    #[prost(uint32, tag = "1")]
    pub block_height: u32,
    /// Within the block, the unique transaction index.
    #[prost(uint32, tag = "2")]
    pub transaction_index: u32,
}

/// MsgCancelOrder is a request type used for canceling orders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelOrder {
    #[prost(message, optional, tag = "1")]
    pub order_id: ::core::option::Option<OrderId>,
    /// Information about when the order cancellation expires.
    #[prost(oneof = "msg_cancel_order::GoodTilOneof", tags = "2, 3")]
    pub good_til_oneof: ::core::option::Option<msg_cancel_order::GoodTilOneof>,
}
/// Nested message and enum types in `MsgCancelOrder`.
pub mod msg_cancel_order {
    /// Information about when the order cancellation expires.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GoodTilOneof {
        /// The last block this order cancellation can be executed at.
        /// Used only for Short-Term orders and must be zero for stateful orders.
        #[prost(uint32, tag = "2")]
        GoodTilBlock(u32),
        /// good_til_block_time represents the unix timestamp (in seconds) at which a
        /// stateful order cancellation will be considered expired. The
        /// good_til_block_time is always evaluated against the previous block's
        /// `BlockTime` instead of the block in which the order is committed.
        /// This value must be zero for Short-Term orders.
        #[prost(fixed32, tag = "3")]
        GoodTilBlockTime(u32),
    }
}



#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateTransfer { 
    #[prost(message, optional, tag = "1")]
    pub transfer: ::core::option::Option<Transfer>,
}

/// Transfer represents a single transfer between two subaccounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    /// The sender subaccount ID.
    #[prost(message, optional, tag = "1")]
    pub sender: ::core::option::Option<SubaccountId>,
    /// The recipient subaccount ID.
    #[prost(message, optional, tag = "2")]
    pub recipient: ::core::option::Option<SubaccountId>,
    /// Id of the asset to transfer.
    #[prost(uint32, tag = "3")]
    pub asset_id: u32,
    /// The amount of asset to transfer
    #[prost(uint64, tag = "4")]
    pub amount: u64,
}
/// MsgDepositToSubaccount represents a single transfer from an `x/bank`
/// account to an `x/subaccounts` subaccount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositToSubaccount {
    /// The sender wallet address.
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// The recipient subaccount ID.
    #[prost(message, optional, tag = "2")]
    pub recipient: ::core::option::Option<SubaccountId>,
    /// Id of the asset to transfer.
    #[prost(uint32, tag = "3")]
    pub asset_id: u32,
    /// The number of quantums of asset to transfer.
    #[prost(uint64, tag = "4")]
    pub quantums: u64,
}
/// MsgWithdrawFromSubaccount represents a single transfer from an
/// `x/subaccounts` subaccount to an `x/bank` account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawFromSubaccount {
    /// The sender subaccount ID.
    #[prost(message, optional, tag = "2")]
    pub sender: ::core::option::Option<SubaccountId>,
    /// The recipient wallet address.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    /// Id of the asset to transfer.
    #[prost(uint32, tag = "3")]
    pub asset_id: u32,
    /// The number of quantums of asset to transfer.
    #[prost(uint64, tag = "4")]
    pub quantums: u64,
}
/// MsgSendFromModuleToAccount represents a single transfer from a module
/// to an `x/bank` account (can be either a module account address or a user
/// account address).
/// Should only be executed by governance.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendFromModuleToAccount {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// The sender module name.
    #[prost(string, tag = "2")]
    pub sender_module_name: ::prost::alloc::string::String,
    /// The recipient account address (can be either a module account address
    /// or a user account address).
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    /// The coin to transfer, which specifies both denom and amount.
    #[prost(message, optional, tag = "4")]
    pub coin: ::core::option::Option<Coin>,
}

/// Coin defines a token with a denomination and an amount.
///
/// NOTE: The amount field is an Int which implements the custom method
/// signatures required by gogoproto.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
