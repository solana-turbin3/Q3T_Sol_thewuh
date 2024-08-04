#[allow(deprecated)]
#[cfg(test)]
mod tests {
    use crate::programs::wba_prereq::{CompleteArgs, WbaPrereqProgram};
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::{
        message::Message,
        native_token::LAMPORTS_PER_SOL,
        signature::{read_keypair_file, Keypair, Signer},
        system_instruction,
        system_program,
        transaction::Transaction,
    };
    use std::{
        env,
        io::{self, BufRead},
        path::Path,
    };

    const RPC_URL: &str = "https://api.devnet.solana.com";

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // get path to keypair json file
        let path = Path::new("dev-wallet.json");

        println!("path: {:#?}", path.canonicalize());

        let keypair = read_keypair_file(path).expect("Couldn't find wallet file");

        println!("{:?}", keypair.to_base58_string());

        let client = RpcClient::new(RPC_URL);

        // request airdrop
        match client.request_airdrop(&keypair.pubkey(), 2 * LAMPORTS_PER_SOL) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
            }
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
        };
    }
    #[test]
    fn transfer() {
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // get path to wba keypair
        // may have unexpected results on windows
        let wallet_path = env::home_dir()
            .expect("could not home dir")
            .join(".config/solana/id.json");

        println!("path: {:#?}", &wallet_path.canonicalize());

        // read wba keypair from file
        let wallet = read_keypair_file(wallet_path).expect("Couldn't find wallet file");

        // Define our WBA public key
        let to_pubkey = &wallet.pubkey();

        // Now let's create a connection to devnet
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // In order to sign transactions, we're going to need to get a recent blockhash, as signatures are
        // designed to expire as a security feature:
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(
                &keypair.pubkey(),
                &to_pubkey,
                LAMPORTS_PER_SOL / 10,
            )],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        // Get balance of dev wallet
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        let message = Message::new_with_blockhash(
            &[system_instruction::transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[system_instruction::transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // Send the transaction
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn enroll() {
        let rpc_client = RpcClient::new(RPC_URL);

        let wallet_path = env::home_dir().unwrap().join(".config/solana/id.json");

        println!("path: {:#?}", &wallet_path.canonicalize());

        let signer = read_keypair_file(wallet_path).expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref(),
        ]);
        // Define our instruction data
        let args = CompleteArgs {
            github: b"thewuhxyz".to_vec(),
        };

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let transaction = WbaPrereqProgram::complete(
            &[&signer.pubkey(), &prereq, &system_program::id()],
            &args,
            Some(&signer.pubkey()),
            &[&signer],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        // Print our transaction out
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }
}
