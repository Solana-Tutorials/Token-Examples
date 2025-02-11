use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, program_pack::Pack, signer::Signer, system_instruction::create_account, transaction::Transaction
};
use spl_token_2022::{instruction::initialize_mint, state::Mint};

use token_examples::{get_or_create_keypair, print_explorer_link, request_and_confirm_airdrop, RPC_URL_LOCAL};
use anyhow::Result;

fn main() -> Result<()> {
    let rpc_url = String::from(RPC_URL_LOCAL);
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer = request_and_confirm_airdrop("WALLET_1", &client)?;
    let mint = get_or_create_keypair("MINT")?;
 
    // Calculate space and rent
    let space = Mint::get_packed_len();
    let lamports = client.get_minimum_balance_for_rent_exemption(space)?;

    // Instruction to create account
    let create_account_instruction = create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        lamports,
        space as u64,
        &spl_token_2022::id(),
    );

    // Instruction to initialize the base mint data
    let initialize_mint_instruction = initialize_mint(
        &spl_token_2022::id(),
        &mint.pubkey(),
        &payer.pubkey(),
        Some(&payer.pubkey()),
        6,
    )?;

    // Create instructions
    let instructions = [
        create_account_instruction,
        initialize_mint_instruction,
    ];

    // Send transaction
    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&payer.pubkey()),
        &[payer, mint],
        recent_blockhash,
    );

    let signature = client.send_and_confirm_transaction(&transaction)?;
    print_explorer_link(&signature);
    Ok(())
}