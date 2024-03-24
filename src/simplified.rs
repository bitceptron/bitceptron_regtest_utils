use bitcoin::Address;
use bitcoincore_rpc::{json::AddressType, Client, RpcApi};

use crate::spawn_regtest::{spawn_regtest, RegtestConf};

pub fn run_one_node() -> Client {
    let client = spawn_regtest(
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bin/bitcoind",
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bitcoin.conf",
        "/Users/bedlam/Desktop/regtemp",
        vec![RegtestConf::new("18446", "18447").unwrap()],
        500,
        2,
    )
    .unwrap()
    .remove(0);
    client
}

pub fn run_one_node_with_test_wallet_loaded() -> Client {
    let client = spawn_regtest(
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bin/bitcoind",
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bitcoin.conf",
        "/Users/bedlam/Desktop/regtemp",
        vec![RegtestConf::new("18446", "18447").unwrap()],
        500,
        2,
    )
    .unwrap()
    .remove(0);
    client
        .create_wallet("test", None, None, None, None)
        .unwrap();
    client
}

pub fn run_one_node_with_test_wallet_loaded_and_mined() -> (Client, Address) {
    let client = spawn_regtest(
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bin/bitcoind",
        "/Users/bedlam/Desktop/getting_rusty/rusties/optech-taproot/bitcoin-26.0/bitcoin.conf",
        "/Users/bedlam/Desktop/regtemp",
        vec![RegtestConf::new("18446", "18447").unwrap()],
        500,
        2,
    )
    .unwrap()
    .remove(0);
    client
        .create_wallet("test", None, None, None, None)
        .unwrap();
    let mining_address = client
        .get_new_address(Some("mining"), Some(AddressType::Bech32))
        .unwrap()
        .require_network(bitcoin::Network::Regtest)
        .unwrap();
    client.generate_to_address(101, &mining_address).unwrap();
    (client, mining_address)
}
