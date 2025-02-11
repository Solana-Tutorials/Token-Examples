use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signer::Signer, transaction::Transaction
};
use spl_token_2022::instruction::transfer_checked;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use token_examples::{get_or_create_keypair, print_explorer_link, request_and_confirm_airdrop, RPC_URL_LOCAL};
use anyhow::Result;

fn main() -> Result<()> {
    let rpc_url = String::from(RPC_URL_LOCAL);
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let wallet_1 = request_and_confirm_airdrop("WALLET_1", &client)?;
    let wallet_2 = get_or_create_keypair("WALLET_2")?;
    let mint = get_or_create_keypair("MINT")?;

    // Instruction to create associated token account
    let wallet_1_token_account = get_associated_token_address_with_program_id(
        &wallet_1.pubkey(), // Token account owner
        &mint.pubkey(),                // Mint
        &spl_token_2022::id(),
    );  

    let wallet_2_token_account = get_associated_token_address_with_program_id(
        &wallet_2.pubkey(), // Token account owner
        &mint.pubkey(),                // Mint
        &spl_token_2022::id(),
    );  

    // Instruction to mint tokens
    let transfer_instruction = transfer_checked(
        &spl_token_2022::id(),
        &wallet_1_token_account,
        &mint.pubkey(),
        &wallet_2_token_account,
        &wallet_1.pubkey(),
        &[&wallet_1.pubkey()],
        100_000,
        6,
    )?;

    // Send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[transfer_instruction],
        Some(&wallet_1.pubkey()),
        &[wallet_1],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    print_explorer_link(&signature);
    Ok(())
}