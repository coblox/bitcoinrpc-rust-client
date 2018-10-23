#[macro_use]
mod from_str;
mod address;
mod block;
mod blockchain;
mod keys;
mod script;
mod serde;
mod transaction;

pub use self::{address::*, block::*, blockchain::*, keys::*, script::*, transaction::*};

use bitcoin::util::hash::Sha256dHash;

// Re-export types from rust-bitcoin so explicit dependency is not necessarily needed
pub type TransactionId = Sha256dHash;
pub type BlockHash = Sha256dHash;

pub use bitcoin::network::constants::Network;
pub use bitcoin::util::privkey::Privkey;
pub use bitcoin::Address;
pub use bitcoin::Script;
pub use bitcoin::Transaction;

#[derive(Deserialize, Serialize, Debug)]
pub struct Account(pub String);

#[allow(non_camel_case_types)]
// TODO: This enum is a bit weird. Clear it up once we have a better understanding of it
#[derive(Deserialize, Serialize, Debug)]
pub enum SigHashType {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ALL|ANYONECANPAY")]
    All_AnyoneCanPay,
    #[serde(rename = "NONE|ANYONECANPAY")]
    None_AnyoneCanPay,
    #[serde(rename = "SINGLE|ANYONECANPAY")]
    Single_AnyoneCanPay,
}
