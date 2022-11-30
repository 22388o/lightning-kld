use crate::bitcoin_manager::BitcoinManager;
use crate::cockroach_manager::CockroachManager;
use crate::manager::Manager;
use std::env::set_var;

pub struct KndManager {
    manager: Manager,
    bin_path: String,
    exporter_address: String,
}

impl KndManager {
    pub async fn start(&mut self) {
        self.manager.start(&self.bin_path, &[]).await
    }

    pub fn pid(&self) -> Option<u32> {
        self.manager.process.as_ref().map(|p| p.id())
    }

    pub async fn call_exporter(&self, method: &str) -> Result<String, reqwest::Error> {
        reqwest::get(format!("http://{}/{}", self.exporter_address, method))
            .await?
            .text()
            .await
    }

    pub fn test_knd(
        output_dir: &str,
        bin_path: &str,
        node_index: u16,
        bitcoin: &BitcoinManager,
        cockroach: &CockroachManager,
    ) -> KndManager {
        let peer_port = 40000u16 + (node_index * 1000u16);
        let exporter_address = format!("127.0.0.1:{}", peer_port + 1);
        let manager = Manager::new(
            output_dir,
            "knd",
            node_index,
            format!("http://{}/health", exporter_address),
        );

        set_var("KND_STORAGE_DIR", &manager.storage_dir);
        set_var("KND_PEER_PORT", &peer_port.to_string());
        set_var("KND_EXPORTER_ADDRESS", &exporter_address);
        set_var("KND_BITCOIN_NETWORK", &bitcoin.network);
        set_var("KND_BITCOIN_COOKIE_PATH", &bitcoin.cookie_path());
        set_var("KND_BITCOIN_RPC_HOST", "127.0.0.1");
        set_var("KND_BITCOIN_RPC_PORT", &bitcoin.rpc_port.to_string());
        set_var("KND_DATABASE_PORT", &cockroach.port.to_string());

        KndManager {
            manager,
            bin_path: bin_path.to_string(),
            exporter_address,
        }
    }
}

#[macro_export]
macro_rules! knd {
    ($bitcoin:expr, $cockroach:expr) => {
        test_utils::knd_manager::KndManager::test_knd(
            env!("CARGO_TARGET_TMPDIR"),
            env!("CARGO_BIN_EXE_lightning-knd"),
            0,
            $bitcoin,
            $cockroach,
        )
    };
    ($n:literal, $bitcoin:expr, $cockroach:expr) => {
        test_utils::knd_manager::KndManager::test_knd(
            env!("CARGO_TARGET_TMPDIR"),
            env!("CARGO_BIN_EXE_lightning-knd"),
            $n,
            $bitcoin,
            $cockroach,
        )
    };
}
