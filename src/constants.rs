///// Primary API /////
// Mainnet
pub const M_RPC: &str = "https://dydx-dao-rpc.polkachu.com:443";
pub const M_REST: &str = "https://dydx-dao-api.polkachu.com:443";
pub const M_GRPC: &str = "http://dydx-dao-grpc-3.polkachu.com:23890";

pub const M_TX_ENDPOINT: &str = "https://dydx-dao-api.polkachu.com:443/cosmos/tx/v1beta1/txs";

// Testnet
pub const T_RPC: &str = "https://dydx-dao-rpc.polkachu.com:443";
pub const T_REST: &str = "https://dydx-dao-api.polkachu.com:443";
pub const T_GRPC: &str = "http://dydx-dao-grpc-3.polkachu.com:23890";

pub const T_TX_ENDPOINT: &str = "https://dydx-dao-api.polkachu.com:443/cosmos/tx/v1beta1/txs";

///// Indexer API /////
// Mainnet
pub const M_IAPI: &str = "https://indexer.dydx.trade/v4";
pub const M_WS: &str = "wss://indexer.dydx.trade/v4/ws";

// Testnet
pub const T_IAPI: &str = "https://dydx-testnet.imperator.co/v4";
pub const T_WS: &str = "wss://indexer.v4testnet.dydx.exchange/v4/ws";

// Cosmos Information
pub const CHILD_PATH: &str = "m/44'/118'/0'/0/0";

///// DYDX Constants /////
pub const STEP_BASE_QUANTUMS: u64 = 1000000;
pub const QUANTUM_CONVERSION_EXPONENT: i8 = -9;
pub const QUOTE_QUANTUMS_ATOMIC_RESOLUTION: i8 = -6;

pub const ORDER_FLAGS_SHORT_TERM: u32 = 0;
pub const ORDER_FLAGS_CONDITIONAL: u32 = 32;
pub const ORDER_FLAGS_LONG_TERM: u32 = 64;
