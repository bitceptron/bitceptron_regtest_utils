use std::{fs, thread::sleep, time::Duration};

use bitcoincore_rpc::{Client, RpcApi};
/// This function stops regtest instances and removes the temp folder.
pub fn unwind_regtest(clients: Vec<Client>, temp_path: &str, sleep_duration_milis: u64) {
    for client in clients {
        client.stop().unwrap();
    }
    sleep(Duration::from_millis(sleep_duration_milis));
    let _ = fs::remove_dir_all(temp_path);
    sleep(Duration::from_millis(sleep_duration_milis));
}
