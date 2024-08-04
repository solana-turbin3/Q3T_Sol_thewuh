import { Keypair } from "@solana/web3.js";
import promptSync from "prompt-sync";
import bs58 from "bs58";

let kp = Keypair.generate();

console.log(`You've generated a new Solana wallet: ${kp.publicKey.toBase58()}`);

console.log(
	`To save your wallet, copy and paste your private key into a JSON file: \n[${kp.secretKey}]`
);

console.log(
	"Congrats, you've created a new Keypair and saved your wallet. Let's go claim some tokens!"
);

const prompt = promptSync();
let base58 = prompt("Enter your wallet:");
let wallet = bs58.encode(Uint8Array.from(JSON.parse(base58)));

console.log("wallet secretkey base58:", wallet);
console.log(
	"wallet publicKey:",
	Keypair.fromSecretKey(bs58.decode(wallet)).publicKey.toBase58()
);
