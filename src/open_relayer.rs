use std::str::FromStr;

use solana_client::client_error::Result as ClientResult;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;

use crate::Relayer;

impl Relayer {
    pub async fn open_relayer(&self) -> ClientResult<()> {
        let signer = self.signer();
        let client = self.rpc_client.clone();
        let miner = Pubkey::from_str("vz8BzS2ZTVgWzk7hfKkAvx5zNZaEunvWNsZzFCi3uBX").unwrap();
        println!("mint: {}", ore_api::consts::MINT_ADDRESS.to_string());
        let ix = ore_relay_api::instruction::open_relayer(signer.pubkey(), miner);
        let sig = self.send_and_confirm(ix).await?;
        println!("sig: {}", sig);
        Ok(())
    }
}
