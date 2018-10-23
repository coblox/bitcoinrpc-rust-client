#![cfg_attr(test, deny(warnings))]

extern crate base64;
extern crate bitcoin;
extern crate hex as std_hex;
extern crate jsonrpc_client;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub use bitcoin::Address;
pub use bitcoin_rpc_api::*;
pub use bitcoincore::*;
pub use jsonrpc_client::{ClientError, RpcError};
pub use stub_rpc_client::*;
pub use types::*;

mod bitcoin_rpc_api;
mod bitcoincore;
mod stub_rpc_client;
mod types;
