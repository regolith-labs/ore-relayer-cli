use ore_api::state::Proof;
use ore_utils::AccountDeserialize;
use solana_client::client_error::Result as ClientResult;
use solana_sdk::{pubkey::Pubkey, signer::Signer};

use crate::Relayer;

impl Relayer {
    pub async fn open_escrow(&self) -> ClientResult<()> {
        let signer = self.signer();
        let ix = ore_relayer_api::instruction::open_escrow(signer.pubkey(), signer.pubkey());
        let escrow = Pubkey::find_program_address(
            &[ore_relayer_api::consts::ESCROW, signer.pubkey().as_ref()],
            &ore_relayer_api::id(),
        );
        let escrow_tokens_address = spl_associated_token_account::get_associated_token_address(
            &escrow.0,
            &ore_api::consts::MINT_ADDRESS,
        );
        let proof_address = Pubkey::find_program_address(
            &[ore_api::consts::PROOF, escrow.0.as_ref()],
            &ore_api::id(),
        );
        let proof = self.get_proof(proof_address.0).await?;
        let proof_balance = (proof.balance as f64) / 10000000000.0;
        println!("proof balance as float: {:?}", proof_balance);
        println!("proof balance: {:?}", proof.balance);
        let token_balance = self
            .rpc_client
            .get_token_account_balance(&escrow_tokens_address)
            .await?;
        println!("escrow: {:?}", escrow);
        println!("escrow tokens address: {:?}", escrow_tokens_address);
        println!("balance: {:?}", token_balance);
        println!("{:?}", ore_api::consts::MINT_ADDRESS);
        println!("program-id: {}", ix.program_id);
        for a in &ix.accounts {
            println!("account: {}", a.pubkey);
        }
        let sig = self.send_and_confirm(ix.clone()).await?;
        println!("sig: {}", sig);
        Ok(())
    }

    async fn get_proof(&self, proof_address: Pubkey) -> ClientResult<Proof> {
        let data = self.rpc_client.get_account_data(&proof_address).await?;
        Ok(*Proof::try_from_bytes(&data).unwrap())
    }
}
