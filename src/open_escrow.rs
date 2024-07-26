use solana_client::client_error::Result as ClientResult;
use solana_sdk::signer::Signer;

use crate::Relayer;

impl Relayer {
    pub async fn open_escrow(&self) -> ClientResult<()> {
        let signer = self.signer();
        let client = self.rpc_client.clone();
        let ix = ore_relayer_api::instruction::open_escrow(signer.pubkey(), signer.pubkey());
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
