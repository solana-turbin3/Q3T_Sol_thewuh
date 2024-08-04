import {
	Commitment,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
} from "@solana/web3.js";
import { wallet } from "./wallet";
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(wallet.secretKey);

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("EvxT6BYjgqvxANU4XFLa5NvvRPacg7L7JX9kHK1dN8S3");

// Recipient address
const to = Keypair.generate().publicKey;

(async () => {
	try {
		// Get the token account of the fromWallet address, and if it does not exist, create it
		const fromTokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			wallet,
			mint,
			wallet.publicKey,
			true,
			"finalized"
		);
		// Get the token account of the toWallet address, and if it does not exist, create it
		const toTokenAccount = await getOrCreateAssociatedTokenAccount(
			connection,
			wallet,
			mint,
			to,
			true,
			"finalized"
		);
		// Transfer the new token to the "toTokenAccount" we just created
		transfer(
			connection,
			wallet,
			fromTokenAccount.address,
			toTokenAccount.address,
			wallet,
			1 * 1000000
		);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
