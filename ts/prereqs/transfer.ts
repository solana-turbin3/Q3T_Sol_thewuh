import {
	Transaction,
	SystemProgram,
	Connection,
	Keypair,
	LAMPORTS_PER_SOL,
	sendAndConfirmTransaction,
	PublicKey,
} from "@solana/web3.js";
import devWallet from "../dev-wallet.json";
import { wallet } from "../cluster1/wallet";

// Import our dev wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(devWallet));
// Define our WBA public key
const to = wallet.publicKey;

//Create a Solana devnet connection
const connection = new Connection("https://api.devnet.solana.com", "finalized");

(async () => {
	try {
		const transaction = new Transaction().add(
			SystemProgram.transfer({
				fromPubkey: from.publicKey,
				toPubkey: to,
				lamports: LAMPORTS_PER_SOL / 10,
			})
		);
		transaction.recentBlockhash = (
			await connection.getLatestBlockhash()
		).blockhash;
		transaction.feePayer = from.publicKey;
		// Sign transaction, broadcast, and confirm
		const signature = await sendAndConfirmTransaction(connection, transaction, [
			from,
		]);
		console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}

	try {
		// Get balance of dev wallet
		const balance = await connection.getBalance(from.publicKey);
		// Create a test transaction to calculate fees
		const transaction = new Transaction().add(
			SystemProgram.transfer({
				fromPubkey: from.publicKey,
				toPubkey: to,
				lamports: balance,
			})
		);
		transaction.recentBlockhash = (
			await connection.getLatestBlockhash()
		).blockhash;
		transaction.feePayer = from.publicKey;
		// Calculate exact fee rate to transfer entire SOL amount out of account minus fees
		const fee =
			(await connection.getFeeForMessage(transaction.compileMessage())).value ||
			0;
		// Remove our transfer instruction to replace it
		transaction.instructions.pop();
		// Now add the instruction back with correct amount of lamports
		transaction.add(
			SystemProgram.transfer({
				fromPubkey: from.publicKey,
				toPubkey: to,
				lamports: balance - fee,
			})
		);
		// Sign transaction, broadcast, and confirm
		const signature = await sendAndConfirmTransaction(connection, transaction, [
			from,
		]);
		console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
