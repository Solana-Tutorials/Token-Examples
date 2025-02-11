use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signer::Signer, transaction::Transaction
};
use anyhow::Result;
use token_examples::{get_or_create_keypair, print_explorer_link, request_and_confirm_airdrop, RPC_URL_LOCAL};
use spl_token_2022::instruction::mint_to;
use spl_associated_token_account::get_associated_token_address_with_program_id;

fn main() -> Result<()> {
    let rpc_url = String::from(RPC_URL_LOCAL);
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let mint_authority = request_and_confirm_airdrop("WALLET_1", &client)?;
    let mint = get_or_create_keypair("MINT")?;

    // Instruction to create associated token account
    let mint_authority_token_account = get_associated_token_address_with_program_id(
        &mint_authority.pubkey(), // Token account owner
        &mint.pubkey(),                // Mint
        &spl_token_2022::id(),
    );  

    // Instruction to mint tokens
    let mint_to_instruction = mint_to(
        &spl_token_2022::id(),
        &mint.pubkey(),                        // Mint
        &mint_authority_token_account,    // Token account to mint to
        &mint_authority.pubkey(),    // Token account owner
        &[&mint_authority.pubkey()], // Additional signers (mint authority)
        100_0000000,                 // Amount to mint
    )?;

    // Send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction],
        Some(&mint_authority.pubkey()),
        &[mint_authority],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    print_explorer_link(&signature);
    Ok(())
}