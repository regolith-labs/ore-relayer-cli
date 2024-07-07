use solana_client::client_error::{ClientError, Result as ClientResult};
use solana_sdk::{
    instruction::Instruction,
    signature::{Signature, Signer},
    transaction::Transaction,
};

use crate::Relayer;

impl Relayer {
    pub async fn send_and_confirm(&self, ix: Instruction) -> ClientResult<Signature> {
        let signer = self.signer();
        let client = self.rpc_client.clone();
        let mut tx = Transaction::new_with_payer(&[ix], Some(&signer.pubkey()));
        let blockhash = client.get_latest_blockhash().await?;
        tx.sign(&[&signer], blockhash);
        client.send_transaction(&tx).await
    }
}
