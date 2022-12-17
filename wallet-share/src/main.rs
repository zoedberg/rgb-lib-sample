use rgb_lib::wallet::{Wallet, WalletData};
use std::sync::Mutex;

pub struct WalletState {
    wallet: Mutex<Wallet>,
}

impl WalletState {
    fn new(wallet_data: WalletData) -> WalletState {
        WalletState {
            wallet: Mutex::new(Wallet::new(wallet_data).expect("valid wallet data")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use once_cell::sync::Lazy;
    use std::sync::MutexGuard;

    static WALLET_STATE: Lazy<WalletState> = Lazy::new(|| {
        println!("This runs only once");
        WalletState::new(
        WalletData {
            data_dir: "/tmp".to_string(),
            bitcoin_network: rgb_lib::BitcoinNetwork::Regtest,
            database_type: rgb_lib::wallet::DatabaseType::Sqlite,
            pubkey: "tpubD6NzVbkrYhZ4YT9CY6kBTU8xYWq2GQPq4NYzaJer1CRrffVLwzYt5Rs3WhjZJGKaNaiN42JfgtnyGwHXc5n5oPbAUSbxwuwDqZci5kdAZHb".to_string(),
            mnemonic: Some("save call film frog usual market noodle hope stomach chat word worry".to_string()),
        })
    });

    fn _get_wallet() -> MutexGuard<'static, Wallet> {
        (*WALLET_STATE).wallet.lock().unwrap()
    }

    #[actix::test]
    async fn test1() {
        let wallet = _get_wallet();
        let addr = wallet.get_address();
        println!("1 ADDRESS: {:?}", addr);
    }

    #[actix::test]
    async fn test2() {
        let wallet = _get_wallet();
        let addr = wallet.get_address();
        println!("2 ADDRESS: {:?}", addr);
    }
}
