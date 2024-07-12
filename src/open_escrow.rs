use solana_client::client_error::Result as ClientResult;
use solana_sdk::signer::Signer;

use crate::{utils::get_relayer, Relayer};

impl Relayer {
    pub async fn open_escrow(&self) -> ClientResult<()> {
        let signer = self.signer();
        let client = self.rpc_client.clone();
        let relayer = get_relayer(&client).await;
        println!("relayer: {:?}", relayer);
        let ix = ore_relay_api::instruction::open_escrow(signer.pubkey(), relayer, signer.pubkey());
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
