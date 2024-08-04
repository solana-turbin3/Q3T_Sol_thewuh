import { wallet } from "./wallet";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";
import path from "path";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(wallet.secretKey);
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
	try {
		//1. Load image
		const imageFile = await readFile(path.join(__dirname, "generug.png"));
		//2. Convert image to generic file.
		const image = createGenericFile(imageFile, "wuh-rug", {
			contentType: "image/png",
		});
		//3. Upload image
		const [myUri] = await umi.uploader.upload([image])
		console.log("Your image URI: ", myUri);
	} catch (error) {
		console.log("Oops.. Something went wrong", error);
	}
})();
