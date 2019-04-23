extern crate bitcoin_rpc_client;
extern crate jsonrpc_client;
#[macro_use]
extern crate log;
extern crate bitcoin;
extern crate hex;
extern crate testcontainers;

use bitcoin_rpc_client::*;
use common::{
    assert::assert_successful_result, test_client::BitcoinCoreTestClient, test_lifecycle::setup,
};
use std::collections::HashMap;

mod common;

#[test]
fn get_address_info() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();

        client.get_address_info(&alice)
    })
}

#[test]
fn add_multisig_address() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();
        let bob = test_client.an_address();

        client.add_multisig_address(1, vec![&alice, &bob])
    })
}

#[test]
fn get_block_count() {
    setup();
    assert_successful_result(BitcoinCoreClient::get_block_count)
}

#[test]
fn get_blockchain_info() {
    setup();
    assert_successful_result(BitcoinCoreClient::get_blockchain_info)
}

#[test]
fn get_new_address() {
    setup();
    assert_successful_result(BitcoinCoreClient::get_new_address)
}

#[test]
fn generate() {
    setup();
    assert_successful_result(|client| client.generate(1))
}

#[test]
fn get_balance() {
    setup();
    assert_successful_result(|client| {
        || client.generate(101).unwrap().unwrap();
        client.get_balance()
    })
}

#[test]
fn list_unspent() {
    setup();
    assert_successful_result(|client| {
        let _ = client.generate(101);
        client.list_unspent(rpc::TxOutConfirmations::Unconfirmed, Some(101), None)
    })
}

#[test]
fn getblock() {
    setup();

    assert_successful_result(|client| {
        let block_hash = BitcoinCoreTestClient::new(client).a_block_hash();

        client.get_block(&block_hash)
    })
}

#[test]
fn get_block_verbose() {
    setup();

    assert_successful_result(|client| {
        let block_hash = BitcoinCoreTestClient::new(client).a_block_hash();
        client.get_block_verbose(&block_hash)
    })
}

#[test]
fn get_best_block_hash() {
    setup();

    assert_successful_result(|client| {
        BitcoinCoreTestClient::new(client).a_block();

        client.get_best_block_hash()
    })
}

#[test]
fn get_block_hash() {
    setup();

    assert_successful_result(|client| {
        BitcoinCoreTestClient::new(client).a_block_hash();

        client.get_block_hash(50)
    })
}

#[test]
fn validate_address() {
    setup();

    assert_successful_result(|client| {
        let address = BitcoinCoreTestClient::new(client).an_address();

        client.validate_address(&address)
    })
}

#[test]
fn validate_multisig_address() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();
        let bob = test_client.an_address();

        let multi_sig = client
            .add_multisig_address(1, vec![&alice, &bob])
            .unwrap()
            .unwrap()
            .address;

        client.validate_address(&multi_sig)
    })
}

#[test]
fn get_raw_transaction_serialized() {
    setup();

    assert_successful_result(|client| {
        let tx_id = BitcoinCoreTestClient::new(client).a_transaction_id();

        client.get_raw_transaction_serialized(&tx_id)
    });
}

#[test]
fn decode_script() {
    setup();

    assert_successful_result(|client| {
        client.decode_script(Script::from(hex::decode("522103ede722780d27b05f0b1169efc90fa15a601a32fc6c3295114500c586831b6aaf2102ecd2d250a76d204011de6bc365a56033b9b3a149f679bc17205555d3c2b2854f21022d609d2f0d359e5bc0e5d0ea20ff9f5d3396cb5b1906aa9c56a0e7b5edc0c5d553ae").unwrap()))
    })
}

#[test]
fn decode_rawtransaction() {
    setup();

    assert_successful_result(|client| {
        client.decode_rawtransaction(rpc::SerializedRawTransaction("0100000001bafe2175b9d7b3041ebac529056b393cf2997f7964485aa382ffa449ffdac02a000000008a473044022013d212c22f0b46bb33106d148493b9a9723adb2c3dd3a3ebe3a9c9e3b95d8cb00220461661710202fbab550f973068af45c294667fc4dc526627a7463eb23ab39e9b01410479be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8ffffffff01b0a86a00000000001976a91401b81d5fa1e55e069e3cc2db9c19e2e80358f30688ac00000000".into()))
    })
}

#[test]
fn decode_rawtransaction_with_txiwnitness() {
    setup();

    assert_successful_result(|client| {
        client.decode_rawtransaction(rpc::SerializedRawTransaction("0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000".into()))
    })
}

#[test]
fn create_raw_transaction() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();
        let _ = test_client.a_block();

        let utxo = test_client.a_utxo();

        let input = rpc::NewTransactionInput::from_utxo(&utxo);
        let mut map = HashMap::new();
        map.insert(alice, utxo.amount);

        client.create_raw_transaction(vec![&input], &map)
    })
}

#[test]
fn dump_privkey() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();

        client.dump_privkey(&alice)
    })
}

#[test]
fn sign_raw_transaction() {
    setup();

    // Note: The signing actually fails but this way, we get to test the deserialization of the datastructures
    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        let alice = test_client.an_address();
        let alice_private_key = test_client.client.dump_privkey(&alice).unwrap().unwrap();

        let utxo = test_client.a_utxo();

        let input = rpc::NewTransactionInput::from_utxo(&utxo);
        let mut map = HashMap::new();
        map.insert(alice, utxo.amount);

        let tx = test_client
            .client
            .create_raw_transaction(vec![&input], &map)
            .unwrap()
            .unwrap();

        client.sign_raw_transaction_with_key(
            &tx,
            Some(vec![&alice_private_key]),
            None,
            Some(rpc::SigHashType::Single_AnyoneCanPay),
        )
    })
}

#[test]
fn send_to_address() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);
        test_client.a_block();
        let alice = test_client.an_address();

        client.send_to_address(&alice, 1.0)
    })
}

#[test]
fn fund_raw_transaction() {
    setup();

    assert_successful_result(|client| {
        let test_client = BitcoinCoreTestClient::new(client);

        test_client.a_block();

        let alice = test_client.an_address();

        let mut outputs = HashMap::new();
        outputs.insert(alice, 10f64);

        let raw_tx = test_client
            .client
            .create_raw_transaction(Vec::new(), &outputs)
            .unwrap()
            .unwrap();
        let options = rpc::FundingOptions::new();

        client.fund_raw_transaction(&raw_tx, &options)
    })
}
