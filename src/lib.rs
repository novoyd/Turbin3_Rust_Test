mod programs;

//Now creating a new keypair over here 
const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use solana_sdk;

#[test]

fn keygen () {
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

    //create a new keypair
    let kp = Keypair::new();
    println!("You'v e generated a new Solana wallet: {}", kp.pubkey().to_string());println!("");
    println!("To save your wallet over here copy paste the following into a JSON file");
    println!("{:?}", kp.to_bytes());
} 
#[test] 
fn airdrop() {}
#[test]
fn transfer_sol() {}
}
#[test]
fn base58_to_wallet() {
    use bs58;
    use std::io::{self,BufRead};
    print!("Input ur priv key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58(){
    use bs58;
    use std::io::{self,BufRead};
    print!("Input your Byte array");
    let stdin = io::stdin();
    let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
    print!("Your private key is");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);

}

#[test]
fn token_airdrop() {
    use solana_client::rpc_client::RpcClient; 
    use solana_sdk::{signature::{Keypair, Signer, read_keypair_file} };
    
    const RPC_URL: &str = "https://api.devnet.solana.com";
    // reading wallet
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 

    let client = RpcClient::new(RPC_URL);

    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! check out your TX here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
        }
        Err(e) => println!("Opps something went wrong: {}", e.to_string())
    }
 
}

#[test]
fn token_transfer() {
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey,system_instruction::transfer};
    use solana_sdk::{message::Message,signature::{Keypair,Signer,read_keypair_file},transaction::Transaction};
    use std::str::FromStr;

    let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet address");

    let to_pubkey = Pubkey::from_str("5N5G7Wt2pumePRNjkfj9BbU8LrAdW4RjswXwNGYx15CM").unwrap();
    //creating devnet connection
    const RPC_URL: &str = "https://api.devnet.solana.com";

    let rpc_client = RpcClient::new(RPC_URL);
    //blockhash for signing txns here
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("failed to get recent blockhash");

    let transaction = Transaction::new_signed_with_payer(&[transfer(&keypair.pubkey(),&to_pubkey,100_000_000)],Some(&keypair.pubkey()),&vec![&keypair],recent_blockhash);

    //sending transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");    

    println!("success our transaction can be found here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

}

#[test]
fn empty_account(){
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file}, 
        transaction::Transaction,
        };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey,system_instruction::transfer};
    use std::str::FromStr;


    // Get balance of dev wallet
const RPC_URL: &str = "https://api.devnet.solana.com";
let to_pubkey = Pubkey::from_str("5N5G7Wt2pumePRNjkfj9BbU8LrAdW4RjswXwNGYx15CM").unwrap();
let rpc_client = RpcClient::new(RPC_URL);
let keypair = read_keypair_file("dev-wallet.json").expect("couldn't find wallet address");

let balance = rpc_client
.get_balance(&keypair.pubkey())
.expect("Failed to get balance");

println!("ok {:?}" ,balance);

let recent_blockhash = rpc_client.get_latest_blockhash().expect("failed to get recent blockhash");

let message = Message::new_with_blockhash(&[transfer(&keypair.pubkey(), &to_pubkey, balance)],Some(&keypair.pubkey()),&recent_blockhash);

let fee = rpc_client.get_fee_for_message(&message).expect("failed to get fee calculator");

let transaction = Transaction::new_signed_with_payer(
    &[transfer( &keypair.pubkey(), &to_pubkey, balance - fee,
    )], Some(&keypair.pubkey()), &vec![&keypair], recent_blockhash);

    
    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaction");    
    println!(
        "Success! Our transaction can be found here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    
}

#[test]
fn enroll(){
    use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs,
        UpdateArgs}; 
    use solana_sdk::{
        message::Message,
        signature::{Keypair, Signer, read_keypair_file}, 
        transaction::Transaction,
        };
    use solana_client::rpc_client::RpcClient;
    use solana_program::{pubkey::Pubkey,system_instruction::transfer,system_program};
 
    
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("Turbin3-wallet.json").expect("couldnt find wallet file");
    let prereq = Turbin3PrereqProgram::derive_program_address(&[b"prereq",signer.pubkey().to_bytes().as_ref()]);

    let args = CompleteArgs {
        github:b"novoyd".to_vec()};
    let blockhash = rpc_client.get_latest_blockhash().expect("Failed to retrieve recent blockhash");

    let transaction = Turbin3PrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()),&[&signer], blockhash);

    let signature = rpc_client.send_and_confirm_transaction(&transaction).expect("Failed to send transaciton");
    println!("Success! Check out your txn here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);

    

}