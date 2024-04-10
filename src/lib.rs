//! This is a tiny Bitcoin Regtest utility crate.
//! 
//! Usage:
//! ```
//! const BITCOIND_PATH: &str = "/Users/x/Desktop/bitcoin-26.0/bin/bitcoind";
//! const BITCOIN_CONF_PATH: &str = "/Users/x/Desktop/bitcoin-26.0/bitcoin.conf";
//! const TEMP_PATH: &str = "/Users/x/Desktop/regtemp";
//! const NETWORK: Network = Network::Regtest;
//! 
//! let mut clients = spawn_regtest(
//!        BITCOIND_PATH,
//!        BITCOIN_CONF_PATH,
//!        TEMP_PATH,
//!        vec![
//!            RegtestConf::new(18447, 18448).unwrap(),
//!            RegtestConf::new(18449, 18450).unwrap(),
//!        ],
//!        500,
//!        2,
//!    )
//!    .unwrap();
//! let mining_client = clients.remove(0);
//! let rusty_client = clients.remove(0);
//! 
//! let mining_address = mining_client
//!        .get_new_address(Some("mining"), Some(AddressType::Bech32))
//!        .unwrap()
//!        .require_network(NETWORK)
//!        .unwrap();
//! let _ = mining_client
//!        .generate_to_address(150, &mining_address)
//!        .unwrap();
//! unwind_regtest(vec![rusty_client, mining_client], TEMP_PATH, 200);
//! ```
mod error;
mod spawn_regtest;
mod common;
mod unwind_regtest;

pub use spawn_regtest::{spawn_regtest, RegtestConf};
pub use unwind_regtest::unwind_regtest;
pub use common::{send_and_mine, get_balance};
pub use error::RegtestUtilsError;
