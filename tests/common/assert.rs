use bitcoin_rpc::BitcoinCoreClient;
use coblox_bitcoincore::BitcoinCore;
use jsonrpc::{HTTPError, RpcResponse};
use std::fmt::Debug;
use testcontainers::{clients::DockerCli, Docker};

pub fn assert_successful_result<R, I>(invocation: I)
where
    R: Debug,
    I: Fn(&BitcoinCoreClient) -> Result<RpcResponse<R>, HTTPError>,
{
    let container = DockerCli::new().run(BitcoinCore::default());
    let client = container.connect(|container| {
        let host_port = container.get_host_port(18443).unwrap();

        let url = format!("http://localhost:{}", host_port);

        let auth = container.image().auth();

        BitcoinCoreClient::new(url.as_str(), auth.username(), auth.password())
    });

    let result = invocation(&client).unwrap().into_result();

    if result.is_err() {
        error!("{:?}", result.unwrap_err());
        panic!("Result should be successful")
    } else {
        // Having a successful result means:
        // - No HTTP Error occured
        // - No deserialization error occured
        debug!("{:?}", result.unwrap())
    }
}
