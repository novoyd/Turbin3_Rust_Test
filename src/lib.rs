

//Now creating a new keypair over here 

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
