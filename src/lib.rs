use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::Signature;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use anyhow::Result;     
use dotenvy::dotenv;

// run command to start local validator: 
// solana-test-validator
pub const RPC_URL_LOCAL: &str = "http://127.0.0.1:8899";
pub const RPC_URL_DEVNET: &str = "https://api.devnet.solana.com";   
const ENV_FILE: &str = ".env";

// Get or create a keypair from an .env file
pub fn get_or_create_keypair(name: &str) -> Result<Keypair> {
    dotenv().ok();

    if let Ok(secret_key_string) = env::var(name) {
        // Try to parse the secret key directly from JSON
        let decoded_secret_key: Vec<u8> = serde_json::from_str(&secret_key_string)?;
        return Ok(Keypair::from_bytes(&decoded_secret_key)?);
    }

    // Create a new keypair and save it
    let keypair = Keypair::new();
    let secret_key_bytes = keypair.to_bytes().to_vec();
    let json_secret_key = serde_json::to_string(&secret_key_bytes)?;

    // Append to .env file
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(ENV_FILE)?
        .write_all(format!("{}={}\n", name, json_secret_key).as_bytes())?;

    Ok(keypair)
}

pub fn request_and_confirm_airdrop(name: &str, client: &RpcClient) -> Result<Keypair> {
    let keypair = get_or_create_keypair(name)?;
    let signature = client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL)?;
    loop {
        if client.confirm_transaction(&signature)? {
            break;
        }
    }

    Ok(keypair)
}

pub fn print_explorer_link(signature: &Signature) {
    println!(
        "\nTransaction: https://explorer.solana.com/tx/{}?cluster=custom",
        signature
    );
}