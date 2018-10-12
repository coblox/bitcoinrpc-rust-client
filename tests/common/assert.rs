use bitcoin_rpc_client::BitcoinCoreClient;
use jsonrpc_client::ClientError;
use jsonrpc_client::RpcError;
use std::fmt::Debug;
use testcontainers::{clients::Cli, images::coblox_bitcoincore::BitcoinCore, Docker};

pub fn assert_successful_result<R, I>(invocation: I)
where
    R: Debug,
    I: Fn(&BitcoinCoreClient) -> Result<Result<R, RpcError>, ClientError>,
{
    let docker = Cli::default();
    let container = docker.run(BitcoinCore::default());
    let client = {
        let host_port = container.get_host_port(18443).unwrap();

        let url = format!("http://localhost:{}", host_port);

        let auth = container.image().auth();

        BitcoinCoreClient::new(url.as_str(), auth.username(), auth.password())
    };

    match invocation(&client) {
        Ok(Ok(result)) => {
            // Having a successful result means:
            // - No HTTP Error occured
            // - No deserialization error occured
            debug!("Returned result: {:?}", result)
        }
        Ok(Err(rpc_error)) => panic!(
            "Network call was successful but node returned rpc-error: {:?}",
            rpc_error
        ),
        Err(http_error) => panic!("Failed to connect to node: {:?}", http_error),
    }
}
