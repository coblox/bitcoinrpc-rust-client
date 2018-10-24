use bitcoin::Address;
use bitcoin::Script;
use types::script::ScriptType;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MultiSigAddress {
    pub address: Address,
    #[serde(rename = "redeemScript")]
    pub redeem_script: Script,
}

/// Most of the Option<T> are due to different address formats
/// Different fields are returned for P2PKH and P2SH addresses.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AddressValidationResult {
    #[serde(rename = "isvalid")]
    is_valid: bool,
    address: Option<Address>,
    #[serde(rename = "scriptPubKey")]
    script_pub_key: Option<String>,
    #[serde(rename = "ismine")]
    is_mine: Option<bool>,
    #[serde(rename = "iswatchonly")]
    is_watch_only: Option<bool>,
    #[serde(rename = "isscript")]
    is_script: Option<bool>,
    #[serde(rename = "script")]
    script_type: Option<ScriptType>,
    #[serde(rename = "hex")]
    redeem_script: Option<Script>,
    addresses: Option<Vec<Address>>,
    #[serde(rename = "sigsrequired")]
    sigs_required: Option<i32>,
    pubkey: Option<String>, //TODO: use PubkeyHash here
    #[serde(rename = "iscompressed")]
    is_compressed: Option<bool>,
    account: Option<String>,
    #[serde(rename = "hdkeypath")]
    hd_key_path: Option<String>,
    #[serde(rename = "hdmasterkeyid")]
    hd_masterkey_id: Option<String>,
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;
    use serde_json;

    #[test]
    fn can_deserialize_mainnet_p2pkh_address() {
        #[derive(Deserialize, Serialize, Debug, PartialEq)]
        struct TestStruct {
            address: Address,
        }

        let address = r#"{"address": "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"}"#;

        let test_struct: TestStruct = serde_json::from_str(address).unwrap();

        assert_eq!(
            test_struct,
            TestStruct {
                address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".parse().unwrap(),
            }
        )
    }

    #[test]
    fn can_deserialize_testnet_p2pkh_address() {
        #[derive(Deserialize, Serialize, Debug, PartialEq)]
        struct TestStruct {
            address: Address,
        }

        let address = r#"{"address": "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn"}"#;

        let test_struct: TestStruct = serde_json::from_str(address).unwrap();

        assert_eq!(
            test_struct,
            TestStruct {
                address: "mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn".parse().unwrap(),
            }
        )
    }

    #[test]
    fn can_deserialize_mainnet_p2sh_address() {
        #[derive(Deserialize, Serialize, Debug, PartialEq)]
        struct TestStruct {
            address: Address,
        }

        let address = r#"{"address": "3EktnHQD7RiAE6uzMj2ZifT9YgRrkSgzQX"}"#;

        let test_struct: TestStruct = serde_json::from_str(address).unwrap();

        assert_eq!(
            test_struct,
            TestStruct {
                address: "3EktnHQD7RiAE6uzMj2ZifT9YgRrkSgzQX".parse().unwrap(),
            }
        )
    }

    #[test]
    fn can_deserialize_testnet_p2sh_address() {
        #[derive(Deserialize, Serialize, Debug, PartialEq)]
        struct TestStruct {
            address: Address,
        }

        let address = r#"{"address": "2MzQwSSnBHWHqSAqtTVQ6v47XtaisrJa1Vc"}"#;

        let test_struct: TestStruct = serde_json::from_str(address).unwrap();

        assert_eq!(
            test_struct,
            TestStruct {
                address: "2MzQwSSnBHWHqSAqtTVQ6v47XtaisrJa1Vc".parse().unwrap(),
            }
        )
    }

    #[test]
    fn can_deserialize_p2pkh_validation_result() {
        let json = r#"
        {
            "isvalid": true,
            "address": "17fshh33qUze2yifiJ2sXgijSMzJ2KNEwu",
            "scriptPubKey": "76a914492ae280d70af33acf0ae7cd329b961e65e9cbd888ac",
            "ismine": true,
            "iswatchonly": false,
            "isscript": false,
            "pubkey": "0312eeb9ae5f14c3cf43cece11134af860c2ef7d775060e3a578ceec888acada31",
            "iscompressed": true,
            "account": "Test"
        }
"#;

        let result: AddressValidationResult = serde_json::from_str(json).unwrap();

        assert_eq!(
            result,
            AddressValidationResult {
                is_valid: true,
                address: Some("17fshh33qUze2yifiJ2sXgijSMzJ2KNEwu".parse().unwrap()),
                script_pub_key: Some(String::from(
                    "76a914492ae280d70af33acf0ae7cd329b961e65e9cbd888ac"
                )),
                is_mine: Some(true),
                is_watch_only: Some(false),
                is_script: Some(false),
                script_type: None,
                redeem_script: None,
                addresses: None,
                sigs_required: None,
                pubkey: Some(String::from(
                    "0312eeb9ae5f14c3cf43cece11134af860c2ef7d775060e3a578ceec888acada31"
                )),
                is_compressed: Some(true),
                account: Some(String::from("Test")),
                hd_key_path: None,
                hd_masterkey_id: None,
            }
        )
    }

    #[test]
    fn can_deserialize_p2sh_validation_result() {
        let json = r#"
        {
            "isvalid" : true,
            "address" : "2MyVxxgNBk5zHRPRY2iVjGRJHYZEp1pMCSq",
            "ismine" : true,
            "iswatchonly" : false,
            "isscript" : true,
            "script" : "multisig",
            "hex" : "522103ede722780d27b05f0b1169efc90fa15a601a32fc6c3295114500c586831b6aaf2102ecd2d250a76d204011de6bc365a56033b9b3a149f679bc17205555d3c2b2854f21022d609d2f0d359e5bc0e5d0ea20ff9f5d3396cb5b1906aa9c56a0e7b5edc0c5d553ae",
            "addresses" : [
                "mjbLRSidW1MY8oubvs4SMEnHNFXxCcoehQ",
                "mo1vzGwCzWqteip29vGWWW6MsEBREuzW94",
                "mt17cV37fBqZsnMmrHnGCm9pM28R1kQdMG"
            ],
            "sigsrequired" : 2,
            "account" : "test account"
        }
"#;

        let result: AddressValidationResult = serde_json::from_str(json).unwrap();

        assert_eq!(result, AddressValidationResult {
            is_valid: true,
            address: Some("2MyVxxgNBk5zHRPRY2iVjGRJHYZEp1pMCSq".parse().unwrap()),
            script_pub_key: None,
            is_mine: Some(true),
            is_watch_only: Some(false),
            is_script: Some(true),
            script_type: Some(ScriptType::MultiSig),
            redeem_script: Some(Script::from(hex::decode("522103ede722780d27b05f0b1169efc90fa15a601a32fc6c3295114500c586831b6aaf2102ecd2d250a76d204011de6bc365a56033b9b3a149f679bc17205555d3c2b2854f21022d609d2f0d359e5bc0e5d0ea20ff9f5d3396cb5b1906aa9c56a0e7b5edc0c5d553ae").unwrap())),
            addresses: Some(vec![
                "mjbLRSidW1MY8oubvs4SMEnHNFXxCcoehQ".parse().unwrap(),
                "mo1vzGwCzWqteip29vGWWW6MsEBREuzW94".parse().unwrap(),
                "mt17cV37fBqZsnMmrHnGCm9pM28R1kQdMG".parse().unwrap(),
            ]),
            sigs_required: Some(2),
            pubkey: None,
            is_compressed: None,
            account: Some(String::from("test account")),
            hd_key_path: None,
            hd_masterkey_id: None,
        })
    }
}
