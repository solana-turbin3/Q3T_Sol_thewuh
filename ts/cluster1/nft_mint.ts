import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createSignerFromKeypair,
	signerIdentity,
	generateSigner,
	percentAmount,
} from "@metaplex-foundation/umi";
import {
	createNft,
	mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";

import { wallet } from "./wallet";
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(wallet.secretKey);
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
	let tx = createNft(umi, {
        mint,
        name: "The Wuh Rug",
        symbol: "WUHRUG",
        sellerFeeBasisPoints: percentAmount(5),
        uri: "https://arweave.net/wrFhKFhdV3H_lyvvqm59uaiwWW2Pt_xRwoS5OtRYe8o",
    })
	let result = await tx.sendAndConfirm(umi);
	const signature = base58.encode(result.signature);

	console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

	console.log("Mint Address: ", mint.publicKey);
})();
