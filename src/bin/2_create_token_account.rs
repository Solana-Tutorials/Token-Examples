use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signer::Signer, transaction::Transaction
};
use spl_associated_token_account::instruction::create_associated_token_account;

use token_examples::{get_or_create_keypair, print_explorer_link, request_and_confirm_airdrop, RPC_URL_LOCAL};
use anyhow::Result;

fn main() -> Result<()> {
    let rpc_url = String::from(RPC_URL_LOCAL);
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let wallet_1 = request_and_confirm_airdrop("WALLET_1", &client)?;
    let wallet_2 = get_or_create_keypair("WALLET_2")?;
    let mint = get_or_create_keypair("MINT")?;
    println!("Wallet 1: {}", wallet_1.pubkey());
    println!("Wallet 2: {}", wallet_2.pubkey());
    println!("Mint: {}", mint.pubkey());


    // Instruction to create associated token account
    let wallet_1_create_associated_token_account_instruction = create_associated_token_account(
        &wallet_1.pubkey(), // Funding account
        &wallet_1.pubkey(), // Token account owner
        &mint.pubkey(),            // Mint
        &spl_token_2022::id(),
    );

    let wallet_2_create_associated_token_account_instruction = create_associated_token_account(
        &wallet_1.pubkey(), // Funding account
        &wallet_2.pubkey(), // Token account owner
        &mint.pubkey(),            // Mint
        &spl_token_2022::id(),
    );

    // Send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[wallet_1_create_associated_token_account_instruction, wallet_2_create_associated_token_account_instruction],
        Some(&wallet_1.pubkey()),
        &[wallet_1],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    print_explorer_link(&signature);
    Ok(())
}