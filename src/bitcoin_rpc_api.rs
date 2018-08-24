use bitcoincore::TxOutConfirmations;
use jsonrpc::HTTPError;
use jsonrpc::RpcError;
use types::*;

pub trait BitcoinRpcApi: Send + Sync {
    // Order as per: https://bitcoin.org/en/developer-reference#rpcs

    fn add_multisig_address(
        &self,
        number_of_required_signatures: u32,
        participants: Vec<&Address>,
    ) -> Result<Result<MultiSigAddress, RpcError>, HTTPError>;

    // TODO: abandontransaction
    // TODO: addmultisigaddress
    // TODO: addnode
    // TODO: addwitnessaddress
    // TODO: backupwallet
    // TODO: bumpfee
    // TODO: clearbanned
    // TODO: createmultisig

    fn create_raw_transaction(
        &self,
        inputs: Vec<&NewTransactionInput>,
        output: &NewTransactionOutput,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError>;

    fn decode_rawtransaction(
        &self,
        tx: SerializedRawTransaction,
    ) -> Result<Result<DecodedRawTransaction, RpcError>, HTTPError>;

    fn decode_script(
        &self,
        script: RedeemScript,
    ) -> Result<Result<DecodedScript, RpcError>, HTTPError>;

    // TODO: disconnectnode

    fn dump_privkey(&self, address: &Address) -> Result<Result<PrivateKey, RpcError>, HTTPError>;

    // TODO: dumpwallet
    // TODO: encryptwallet
    // TODO: estimatefee
    // TODO: estimatepriority

    fn fund_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        options: &FundingOptions,
    ) -> Result<Result<FundingResult, RpcError>, HTTPError>;

    fn generate(
        &self,
        number_of_blocks: u32,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, HTTPError>;

    // TODO: generatetoaddress
    // TODO: getaccountaddress

    fn get_account(&self, address: &Address) -> Result<Result<Account, RpcError>, HTTPError>;

    // TODO: getaddednodeinfo
    // TODO: getaddressesbyaccount
    // TODO: getbalance
    // TODO: getbestblockhash

    fn get_block(&self, header_hash: &BlockHash) -> Result<Result<Block, RpcError>, HTTPError>;

    fn get_blockchain_info(&self) -> Result<Result<BlockchainInfo, RpcError>, HTTPError>;

    fn get_block_count(&self) -> Result<Result<BlockHeight, RpcError>, HTTPError>;

    // TODO: getblockhash
    // TODO: getblockheader
    // TODO: getblocktemplate
    // TODO: getchaintips
    // TODO: getconnectioncount
    // TODO: getdifficulty
    // TODO: getgenerate
    // TODO: gethashespersec
    // TODO: getinfo
    // TODO: getmemoryinfo
    // TODO: getmempoolancestors
    // TODO: getmempooldescendants
    // TODO: getmempoolentry
    // TODO: getmempoolinfo
    // TODO: getmininginfo
    // TODO: getnettotals
    // TODO: getnetworkhashesps
    // TODO: getnetworkinfo

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, HTTPError>;

    // TODO: getpeerinfo
    // TODO: getrawchangeaddress
    // TODO: getrawmempool

    fn get_raw_transaction_serialized(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, HTTPError>;

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<VerboseRawTransaction, RpcError>, HTTPError>;

    // TODO: getreceivedbyaccount
    // TODO: getreceivedbyaddress

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, HTTPError>;

    // TODO: gettxout
    // TODO: gettxoutsetinfo
    // TODO: getunconfirmedbalance
    // TODO: getwalletinfo
    // TODO: getwork
    // TODO: importaddress
    // TODO: importmulti
    // TODO: importprivkey
    // TODO: importprunedfunds
    // TODO: importwallet
    // TODO: keypoolrefill
    // TODO: invalidateblock
    // TODO: keypoolrefill
    // TODO: listaccounts
    // TODO: listaddressgroupings
    // TODO: listbanned
    // TODO: listlockunspent
    // TODO: listreceivedbyaccount
    // TODO: listreceivedbyaddress
    // TODO: listsinceblock
    // TODO: listtransactions

    fn list_unspent(
        &self,
        min_confirmations: TxOutConfirmations,
        max_confirmations: Option<u32>,
        recipients: Option<Vec<Address>>,
    ) -> Result<Result<Vec<UnspentTransactionOutput>, RpcError>, HTTPError>;

    // TODO: lockunspent
    // TODO: move
    // TODO: ping
    // TODO: preciousblock
    // TODO: prioritisetransaction
    // TODO: pruneblockchain
    // TODO: removeprunedfunds
    // TODO: sendfrom
    // TODO: sendmany

    fn send_raw_transaction(
        &self,
        tx_data: SerializedRawTransaction,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError>;

    fn send_to_address(
        &self,
        address: &Address,
        amount: f64,
    ) -> Result<Result<TransactionId, RpcError>, HTTPError>;
    // TODO: setaccount
    // TODO: setban
    // TODO: setgenerate
    // TODO: setnetworkactive
    // TODO: settxfee
    // TODO: signmessage
    // TODO: signmessagewithprivkey

    fn sign_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        dependencies: Option<Vec<&TransactionOutputDetail>>,
        private_keys: Option<Vec<&PrivateKey>>,
        signature_hash_type: Option<SigHashType>,
    ) -> Result<Result<SigningResult, RpcError>, HTTPError>;

    // TODO: stop
    // TODO: submitblock

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<AddressValidationResult, RpcError>, HTTPError>;

    // TODO: verifychain
    // TODO: verifymessage
    // TODO: verifytxoutproof
    // TODO: walletlock
    // TODO: walletpassphrase
    // TODO: walletpassphrasechange
}
