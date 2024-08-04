import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import {
	getOrCreateAssociatedTokenAccount,
	mintTo,
	mintToChecked,
} from "@solana/spl-token";
import { wallet } from "./wallet";

// Import our keypair from the wallet file
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000;

// Mint address
const mint = new PublicKey("EvxT6BYjgqvxANU4XFLa5NvvRPacg7L7JX9kHK1dN8S3");

(async () => {
	try {
		const destination = Keypair.generate();

		// Create an ATA
		const destinationAta = await getOrCreateAssociatedTokenAccount(
			connection,
			wallet,
			mint,
			destination.publicKey
		);
		console.log(`Your ata is: ${destinationAta.address.toBase58()}`);

		// Mint to ATA
		const mintTx = await mintToChecked(
			connection,
			wallet,
			mint,
			destinationAta.address,
			wallet,
			100*token_decimals,
			6
		);

		console.log(`Your mint txid: ${mintTx}`);
	} catch (error) {
		console.log(`Oops, something went wrong: ${error}`);
	}
})();
