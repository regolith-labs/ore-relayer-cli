use cached::proc_macro::cached;
use ore_api::{consts::PROOF, state::Proof};
use ore_relay_api::state::Relayer;
use ore_utils::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

pub async fn get_proof(client: &RpcClient, authority: Pubkey) -> Proof {
    let proof_address = proof_pubkey(authority);
    let data = client
        .get_account_data(&proof_address)
        .await
        .expect("Failed to get miner account");
    *Proof::try_from_bytes(&data).expect("Failed to parse miner account")
}

pub async fn get_relayer(client: &RpcClient) -> Relayer {
    let relayer_address = relayer_pubkey();
    let data = client
        .get_account_data(&relayer_address)
        .await
        .expect("Failed to get relayer account");
    *Relayer::try_from_bytes(&data).expect("Failed to parse realyer account")
}

#[cached]
pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ore_api::ID).0
}

#[cached]
pub fn relayer_pubkey() -> Pubkey {
    Pubkey::find_program_address(
        &[
            ore_relay_api::consts::RELAYER,
            ore_relay_api::consts::AUTHORIZED_RELAYER.as_ref(),
        ],
        &ore_relay_api::id(),
    )
    .0
}
