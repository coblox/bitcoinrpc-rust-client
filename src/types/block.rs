use BlockHash;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct BlockHeight(u32);

impl BlockHeight {
    pub fn new(h: u32) -> BlockHeight {
        BlockHeight(h)
    }

    pub fn as_i64(&self) -> i64 {
        i64::from(self.0)
    }
}

impl From<BlockHeight> for u32 {
    fn from(block_height: BlockHeight) -> Self {
        block_height.0
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Block<T> {
    pub hash: BlockHash,
    pub confirmations: i32,
    pub size: u32,
    pub strippedsize: u32,
    pub weight: u32,
    pub height: u32,
    pub version: u32,
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    pub merkleroot: String,
    pub tx: Vec<T>,
    pub time: u64,
    pub mediantime: u64,
    pub nonce: u32,
    pub bits: String,
    pub difficulty: f64,
    pub chainwork: String,
    pub previousblockhash: Option<BlockHash>,
    pub nextblockhash: Option<BlockHash>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin_hashes::hex::FromHex;
    use serde_json;
    use TransactionId;

    #[test]
    fn can_deserialize_block_struct() {
        let json = r#"{
	"hash": "00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048",
	"confirmations": 447014,
	"strippedsize": 215,
	"size": 215,
	"weight": 860,
	"height": 1,
	"version": 1,
	"versionHex": "00000001",
	"merkleroot": "0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098",
	"tx": [
		"0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"
	],
	"time": 1231469665,
	"mediantime": 1231469665,
	"nonce": 2573394689,
	"bits": "1d00ffff",
	"difficulty": 1,
	"chainwork": "0000000000000000000000000000000000000000000000000000000200020002",
	"previousblockhash": "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
	"nextblockhash": "000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd"
}"#;
        let block: Block<TransactionId> = serde_json::from_str(json).unwrap();

        assert_eq!(
            block,
            Block {
                hash: BlockHash::from_hex(
                    "00000000839a8e6886ab5951d76f411475428afc90947ee320161bbf18eb6048"
                )
                .unwrap(),
                confirmations: 447014,
                size: 215,
                strippedsize: 215,
                weight: 860,
                height: 1,
                version: 1,
                version_hex: "00000001".to_string(),
                merkleroot: "0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098"
                    .to_string(),
                tx: vec![TransactionId::from_hex(
                    "0e3e2357e806b6cdb1f70b54c3a3a17b6714ee1f0e68bebb44a74b1efd512098",
                )
                .unwrap(),],
                time: 1231469665,
                mediantime: 1231469665,
                nonce: 2573394689,
                bits: "1d00ffff".to_string(),
                difficulty: 1.0,
                chainwork: "0000000000000000000000000000000000000000000000000000000200020002"
                    .to_string(),
                previousblockhash: Some(
                    BlockHash::from_hex(
                        "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f",
                    )
                    .unwrap()
                ),
                nextblockhash: Some(
                    BlockHash::from_hex(
                        "000000006a625f06636b8bb6ac7b960a8d03705d1ace08b1a19da3fdcc99ddbd",
                    )
                    .unwrap()
                ),
            }
        )
    }
}
