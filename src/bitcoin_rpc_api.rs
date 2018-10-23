use bitcoin::Address;
use bitcoincore::TxOutConfirmations;
use jsonrpc_client::ClientError;
use jsonrpc_client::RpcError;
use types::*;

#[allow(unused_variables)]
pub trait BitcoinRpcApi: Send + Sync {
    // Order as per: https://bitcoin.org/en/developer-reference#rpcs

    fn add_multisig_address(
        &self,
        number_of_required_signatures: u32,
        participants: Vec<&Address>,
    ) -> Result<Result<MultiSigAddress, RpcError>, ClientError> {
        unimplemented!()
    }

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
    ) -> Result<Result<SerializedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn decode_rawtransaction(
        &self,
        tx: SerializedRawTransaction,
    ) -> Result<Result<DecodedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn decode_script(
        &self,
        script: RedeemScript,
    ) -> Result<Result<DecodedScript, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: disconnectnode

    fn dump_privkey(&self, address: &Address) -> Result<Result<PrivateKey, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: dumpwallet
    // TODO: encryptwallet
    // TODO: estimatefee
    // TODO: estimatepriority

    fn fund_raw_transaction(
        &self,
        tx: &SerializedRawTransaction,
        options: &FundingOptions,
    ) -> Result<Result<FundingResult, RpcError>, ClientError> {
        unimplemented!()
    }

    fn generate(
        &self,
        number_of_blocks: u32,
    ) -> Result<Result<Vec<BlockHash>, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: generatetoaddress
    // TODO: getaccountaddress

    fn get_account(&self, address: &Address) -> Result<Result<Account, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getaddednodeinfo
    // TODO: getaddressesbyaccount
    // TODO: getbalance

    fn get_best_block_hash(&self) -> Result<Result<BlockHash, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<Block<TransactionId>, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO(evg): add verbosity param to get_block instead of separate methods?
    fn get_block_verbose(
        &self,
        header_hash: &BlockHash,
    ) -> Result<Result<Block<DecodedRawTransaction>, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_blockchain_info(&self) -> Result<Result<BlockchainInfo, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block_count(&self) -> Result<Result<BlockHeight, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_block_hash(&self, height: u32) -> Result<Result<BlockHash, RpcError>, ClientError> {
        unimplemented!()
    }

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

    fn get_new_address(&self) -> Result<Result<Address, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getpeerinfo
    // TODO: getrawchangeaddress
    // TODO: getrawmempool

    fn get_raw_transaction_serialized(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<SerializedRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    fn get_raw_transaction_verbose(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<VerboseRawTransaction, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: getreceivedbyaccount
    // TODO: getreceivedbyaddress

    fn get_transaction(
        &self,
        tx: &TransactionId,
    ) -> Result<Result<Transaction, RpcError>, ClientError> {
        unimplemented!()
    }

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
    ) -> Result<Result<Vec<UnspentTransactionOutput>, RpcError>, ClientError> {
        unimplemented!()
    }

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
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
        unimplemented!()
    }

    fn send_to_address(
        &self,
        address: &Address,
        amount: f64,
    ) -> Result<Result<TransactionId, RpcError>, ClientError> {
        unimplemented!()
    }
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
    ) -> Result<Result<SigningResult, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: stop
    // TODO: submitblock

    fn validate_address(
        &self,
        address: &Address,
    ) -> Result<Result<AddressValidationResult, RpcError>, ClientError> {
        unimplemented!()
    }

    // TODO: verifychain
    // TODO: verifymessage
    // TODO: verifytxoutproof
    // TODO: walletlock
    // TODO: walletpassphrase
    // TODO: walletpassphrasechange
}
