use crate::{
    constants::*,
    Subaccount,
};

pub(crate) mod faucet;
pub(crate) mod indexer;
pub(crate) mod validator;
pub(crate) mod websocket;
pub(crate) mod composite;
pub(crate) mod mappings;
pub mod responses;

pub struct CompositeClient {
    pub indexer: IndexerClient,
    pub validator: ValidatorClient,
}

pub struct IndexerClient {
    pub endpoint: String,
}

pub struct ValidatorClient {
    // pub subaccount: Subaccount,
    pub endpoints: Endpoints,
}

pub struct FaucetClient {
    pub endpoint: String,
}

pub struct WebsocketClient {
    pub endpoint: String,
}

#[derive(Clone)]
pub struct Endpoints {
    pub rest: String,
    pub rpc: String,
    pub grpc: String,
}

impl Default for Endpoints {

    fn default() -> Self {
        Self {
            rest: String::new(),
            rpc: String::new(),
            grpc: String::new(),
        }
    }
}

impl Endpoints {

    pub fn new<T: ToString>(rest: T, rpc: T, grpc: T) -> Self {
        Self {
            rest: rest.to_string(),
            rpc: rpc.to_string(),
            grpc: grpc.to_string(),
        }
    }

    pub fn mainnet() -> Self {
        Self::new(M_REST, M_REST, M_REST)
    }

    pub fn testnet() -> Self {
        Self::new(T_REST, T_REST, T_REST)
    }
}
