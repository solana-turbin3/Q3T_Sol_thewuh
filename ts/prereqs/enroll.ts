import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, WbaPrereq } from "../programs/wba_prereq";
import fs from "fs";
import path from "path";
import os from "os";

const filePath = path.join(os.homedir(), ".config", "solana", "id.json");

const walletString = fs.readFileSync(filePath, "utf8");

const wallet = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(walletString)));

console.log("wallet:", wallet.publicKey.toBase58());

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Github account
const github = Buffer.from("thewuhxyz", "utf8");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(wallet), {
	commitment: "confirmed",
});

// Create our program
const program: Program<WbaPrereq> = new Program(IDL as WbaPrereq, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), wallet.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
	enrollment_seeds,
	program.programId
);
// Execute our enrollment transaction
(async () => {
	try {
		const txhash = await program.methods
			.complete(github)
			.accountsStrict({
				signer: wallet.publicKey,
				systemProgram: SystemProgram.programId,
				prereq: enrollment_key,
			})
			.signers([wallet])
			.rpc();
		console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
