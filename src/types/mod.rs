mod serde;

pub mod address;
pub mod block;
pub mod blockchain;
pub mod keys;
pub mod script;
pub mod transaction;

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

pub enum TxOutConfirmations {
    Unconfirmed,
    AtLeast(i32),
}
