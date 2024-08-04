import {wallet} from "./wallet";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createGenericFile,
	createSignerFromKeypair,
	signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Create a devnet connection
const umi = createUmi("https://api.devnet.solana.com");

let keypair = umi.eddsa.createKeypairFromSecretKey(wallet.secretKey);
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
	try {
		// Follow this JSON structure
		// https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

		const image =
			"https://arweave.net/93vYhMFvtt1Nzt5nSDa0QNkKkO_6pd7nN3aQeE-SVF8";
		const metadata = {
			name: "THE WUH RUG",
			symbol: "WUHRUG",
			description: "the wuh rug",
			image,
			attributes: [{ trait_type: "material", value: "fur" }],
			properties: {
				files: [
					{
						type: "image/png",
						uri: image,
					},
				],
			},
			creators: [],
		};
		const myUri = await umi.uploader.uploadJson(metadata);
		console.log("Your image URI: ", myUri);
	} catch (error) {
		console.log("Oops.. Something went wrong", error);
	}
})();
