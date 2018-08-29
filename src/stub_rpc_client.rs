use bitcoin_rpc_api::BitcoinRpcApi;

pub struct BitcoinStubClient {}

impl BitcoinStubClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl BitcoinRpcApi for BitcoinStubClient {}
