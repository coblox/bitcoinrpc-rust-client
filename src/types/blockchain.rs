use bitcoin::network::constants::Network;
use types::*;

#[derive(Deserialize, Debug, PartialEq)]
pub struct SoftFork {
    pub id: String,
    pub version: u32,
    pub reject: Reject,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Reject {
    pub status: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Bip9SoftFork {
    pub csv: Bip9SoftForkDetails,
    pub segwit: Bip9SoftForkDetails,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Bip9SoftForkDetails {
    pub status: String,
    pub bit: Option<u32>,
    #[serde(rename = "startTime")]
    // In regtest, startTime is -1
    pub start_time: i64,
    pub timeout: u64,
    pub since: u64,
    // TODO: implement before new BIP9
    /*
    "statistics": {         (object) numeric statistics about BIP9 signalling for a softfork (only for "started" status)
    "period": xx,        (numeric) the length in blocks of the BIP9 signalling period
    "threshold": xx,     (numeric) the number of blocks with the version bit set required to activate the feature
    "elapsed": xx,       (numeric) the number of blocks elapsed since the beginning of the current period
    "count": xx,         (numeric) the number of blocks with the version bit set in the current period
    "possible": xx       (boolean) returns false if there are not enough blocks left in this period to pass activation threshold
    */
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct BlockchainInfo {
    #[serde(with = "self::serde::network")]
    pub chain: Network,
    pub blocks: u64,
    pub headers: u64,
    pub bestblockhash: String,
    //TODO: Cannot trust serde - it is not able to deserialise “4.656542373906925e-10"
    pub difficulty: f64,
    pub mediantime: u64,
    pub verificationprogress: f64,
    pub initialblockdownload: bool,
    pub chainwork: String,
    pub size_on_disk: u64,
    pub pruned: bool,
    pub pruneheight: Option<u64>,
    pub automatic_pruning: Option<bool>,
    pub prune_target_size: Option<u64>,
    pub softforks: Vec<SoftFork>,
    pub bip9_softforks: Bip9SoftFork,
    pub warnings: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn can_deserialize_blockchain_response() {
        let json = r#"{
        "chain": "regtest",
        "blocks": 0,
        "headers": 0,
        "bestblockhash": "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206",
        "difficulty": 4.65654237390692e-10,
        "mediantime": 1296688602,
        "verificationprogress": 1,
        "initialblockdownload": true,
        "chainwork": "0000000000000000000000000000000000000000000000000000000000000002",
        "size_on_disk": 293,
        "pruned": false,
        "softforks": [
            {
                "id": "bip34",
                "version": 2,
                "reject": {
                    "status": false
                }
            },
            {
                "id": "bip66",
                "version": 3,
                "reject": {
                    "status": false
                }
            },
            {
                "id": "bip65",
                "version": 4,
                "reject": {
                    "status": false
                }
            }
        ],
        "bip9_softforks": {
            "csv": {
                "status": "defined",
                "startTime": 0,
                "timeout": 9223372036854775807,
                "since": 0
            },
            "segwit": {
                "status": "active",
                "startTime": -1,
                "timeout": 9223372036854775807,
                "since": 0
            }
        },
        "warnings": ""
}"#;
        let blockchain: BlockchainInfo = serde_json::from_str(json).unwrap();

        assert_eq!(
            blockchain,
            BlockchainInfo {
                chain: Network::Regtest,
                blocks: 0,
                headers: 0,
                bestblockhash: String::from(
                    "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"
                ),
                difficulty: 4.65654237390692e-10,
                mediantime: 1296688602,
                verificationprogress: 1.0,
                initialblockdownload: true,
                chainwork: String::from(
                    "0000000000000000000000000000000000000000000000000000000000000002"
                ),
                size_on_disk: 293,
                pruned: false,
                pruneheight: None,
                automatic_pruning: None,
                prune_target_size: None,
                softforks: vec![
                    SoftFork {
                        id: String::from("bip34"),
                        version: 2,
                        reject: Reject { status: false },
                    },
                    SoftFork {
                        id: String::from("bip66"),
                        version: 3,
                        reject: Reject { status: false },
                    },
                    SoftFork {
                        id: String::from("bip65"),
                        version: 4,
                        reject: Reject { status: false },
                    },
                ],
                bip9_softforks: Bip9SoftFork {
                    csv: Bip9SoftForkDetails {
                        status: String::from("defined"),
                        bit: None,
                        start_time: 0,
                        timeout: 9223372036854775807,
                        since: 0,
                    },
                    segwit: Bip9SoftForkDetails {
                        status: String::from("active"),
                        bit: None,
                        start_time: -1,
                        timeout: 9223372036854775807,
                        since: 0,
                    },
                },
                warnings: String::new(),
            },
        )
    }
}
