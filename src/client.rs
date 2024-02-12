use crate::constants::*;

pub struct NetworkConfig {
    rest: String,
    rpc: String,
}

impl NetworkConfig {

    pub fn new(rest: &str, rpc: &str) -> Self {
        Self {
            rest: rest.to_string(),
            rpc: rpc.to_string(),
        }
    }

    pub fn mainnet() -> Self {
        Self {
            rest: M_REST.to_string(),
            rpc: M_RPC.to_string(),
        }
    }

    pub fn testnet() -> Self {
        Self {
            rest: T_REST.to_string(),
            rpc: T_RPC.to_string(),
        }
    }
}

pub trait DydxClient {

}

pub(crate) struct IndexerClient {
    config: NetworkConfig,
}

pub(crate) struct ValidatorClient {

}

pub(crate) struct FaucetClient {

}

pub(crate) struct WebsocketClient {
    websocket: String,
}

impl DydxClient for IndexerClient {}
impl DydxClient for ValidatorClient {}
impl DydxClient for FaucetClient {}

impl IndexerClient {

    pub fn rest(&self) -> &str {
        &self.config.rest
    }
}

impl WebsocketClient {

    pub fn websocket(&self) -> &str {
        &self.websocket
    }
}
