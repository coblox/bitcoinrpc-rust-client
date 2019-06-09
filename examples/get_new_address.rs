#![allow(deprecated)]
extern crate bitcoin_rpc_client;

use bitcoin_rpc_client::BitcoinCoreClient;
use bitcoin_rpc_client::BitcoinRpcApi;
use std::env::var;

fn main() {
    let url = var("BITCOIN_CORE_URL").unwrap();
    let user = var("BITCOIN_CORE_USER").unwrap();
    let password = var("BITCOIN_CORE_PASSWORD").unwrap();

    let client = BitcoinCoreClient::new(&url, &user, &password);

    let address = client
        .get_new_address()
        .unwrap() // Handle network error here
        .unwrap(); // Handle RpcError

    println!("Generated address: {:?}", address);
}
