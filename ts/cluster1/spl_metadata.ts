import { wallet } from "./wallet";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
	createMetadataAccountV3,
	CreateMetadataAccountV3InstructionAccounts,
	CreateMetadataAccountV3InstructionArgs,
	DataV2Args,
} from "@metaplex-foundation/mpl-token-metadata";
import {
	createSignerFromKeypair,
	signerIdentity,
	publicKey,
} from "@metaplex-foundation/umi";
import bs58 from "bs58";

// Define our Mint address
const mint = publicKey("EvxT6BYjgqvxANU4XFLa5NvvRPacg7L7JX9kHK1dN8S3");

// Create a UMI connection
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(wallet.secretKey);
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
	try {
		// Start here
		let accounts: CreateMetadataAccountV3InstructionAccounts = {
			mint,
			mintAuthority: signer,
		};

		let data: DataV2Args = {
			collection: null,
			creators: [],
			name: "WUH TOKEN",
			sellerFeeBasisPoints: 0,
			symbol: "WUH",
			uri: "",
			uses: null,
		};

		let args: CreateMetadataAccountV3InstructionArgs = {
			data,
			isMutable: true,
			collectionDetails: null,
		};

		let tx = createMetadataAccountV3(umi, {
			...accounts,
			...args,
		});

		let result = await tx.sendAndConfirm(umi);
		console.log(bs58.encode(result.signature));
	} catch (e) {
		console.error(`Oops, something went wrong: ${e}`);
	}
})();
