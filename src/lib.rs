#![cfg_attr(test, deny(warnings))]

extern crate base64;
extern crate bitcoin;
extern crate bitcoin_hashes;
extern crate bitcoin_quantity;
extern crate hex as std_hex;
extern crate jsonrpc_client;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod bitcoin_rpc_api;
mod bitcoincore;
mod stub_rpc_client;
mod types;

// Re-export types from rust-bitcoin crates so explicit dependency is not               needed
pub type TransactionId = bitcoin_hashes::sha256d::Hash;
pub type BlockHash = bitcoin_hashes::sha256d::Hash;

pub use bitcoin::network::constants::Network;
pub use bitcoin::util::key::PrivateKey;
pub use bitcoin::Address;
pub use bitcoin::Script;

pub use bitcoin_rpc_api::BitcoinRpcApi;
pub use bitcoincore::BitcoinCoreClient;
pub use stub_rpc_client::BitcoinStubClient;

pub use jsonrpc_client::{ClientError, RpcError};

pub mod rpc {
    pub use types::address::*;
    pub use types::block::*;
    pub use types::blockchain::*;
    pub use types::keys::*;
    pub use types::script::*;
    pub use types::transaction::*;
    pub use types::{Account, SigHashType, TxOutConfirmations};
}
