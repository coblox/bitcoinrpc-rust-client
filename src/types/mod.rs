#[macro_use]
mod from_str;
mod address;
mod block;
mod blockchain;
mod keys;
mod script;
mod serde;
mod transaction;

use bitcoin::util::hash::{Sha256dHash, HexError};

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct BlockHash(String);

impl BlockHash {
    pub fn into_sha256d_hash(self) -> Result<Sha256dHash, HexError> {
        let rev = Sha256dHash::from_hex(&self.0).unwrap();
        Ok(Sha256dHash::from(rev.into_bytes().into_iter().map(|x| *x).rev().collect::<Vec<u8>>().as_slice()))
    }
}

from_str!(BlockHash);

#[derive(Deserialize, Serialize, Debug)]
pub struct Account(String);

from_str!(Account);

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

pub use self::{address::*, block::*, blockchain::*, keys::*, script::*, transaction::*};
