use regtest_utils::spawn_regtest::spawn_regtest;
use regtest_utils::spawn_regtest::RegtestConf;
use regtest_utils::unwind_regtest::unwind_regtest;

#[tokio::main]
async fn main() {
    let bitcoind_path =
        "/bitcoin-26.0/bin/bitcoind";
    let bitcoin_conf_path =
        "/bitcoin-26.0/bitcoin.conf";
    let temp_path = "/regtemp";
    let conf1 = RegtestConf::new("18445", "18446").unwrap();
    let conf2 = RegtestConf::new("18447", "18448").unwrap();
    let conf3 = RegtestConf::new("18449", "18450").unwrap();

    let clients = spawn_regtest(
        bitcoind_path,
        bitcoin_conf_path,
        temp_path,
        vec![conf1, conf2, conf3],
        100,
        20,
    )
    .unwrap();

    unwind_regtest(clients, temp_path);
}
