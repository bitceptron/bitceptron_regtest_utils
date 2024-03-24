use bitcoincore_rpc::Client;

use crate::spawn_regtest::{spawn_regtest, RegtestConf};

pub fn run_one_node() -> Client{
    let client = spawn_regtest(
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bin/bitcoind",
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bitcoin.conf",
        "/Users/bedlam/Desktop/regtemp",
        vec![RegtestConf::new("18446", "18447").unwrap()],
        500,
        2,
    ).unwrap().remove(0);
    client
}
