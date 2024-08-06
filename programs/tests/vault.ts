import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import {
	Cluster,
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
} from "@solana/web3.js";
import { assert } from "chai";

describe("vault", () => {
	// Configure the client to use the local cluster.

	anchor.setProvider(anchor.AnchorProvider.env());

	const program = anchor.workspace.Vault as Program<Vault>;
	const provider = program.provider;
	const connection = provider.connection;

	const user = Keypair.generate();
  console.log("user:", user.publicKey.toBase58())

	const vaultState = PublicKey.findProgramAddressSync(
		[Buffer.from("state"), user.publicKey.toBuffer()],
		program.programId
	)[0];

  console.log("vault state:", vaultState.toBase58())
  
	const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBuffer()],
		program.programId
	)[0];
  
  console.log("vault:", vault.toBase58())
	const txHelper = (tx: string, cluster: Cluster | "custom" = "devnet") =>
		`https://explorer.solana.com/tx/${tx}${
			cluster === "mainnet-beta" ? "" : `?cluster=${cluster}`
		}`;

	const sleep = (sec: number) =>
		new Promise((resolve) => setTimeout(resolve, sec * 1000));

	before(async () => {
		let tx = await connection.requestAirdrop(
			user.publicKey,
			1 * LAMPORTS_PER_SOL
		);
		console.log("✅ Transaction successful:", txHelper(tx, "custom"));

    await sleep(5)
	});

	it("Initializes vault", async () => {
		// Add your test here.
		const tx = await program.methods
			.initialize()
			.accountsStrict({
				systemProgram: SystemProgram.programId,
				user: user.publicKey,
				vault,
				vaultState,
			})
			.signers([user])
			.rpc();
		console.log("Your transaction signature", tx);

    await sleep(5);
	});

	it("deposits to vault", async () => {
		const beforeUserBalance = await connection.getBalance(user.publicKey);

		// Add your test here.
		const tx = await program.methods
			.deposit(new anchor.BN(0.2 * LAMPORTS_PER_SOL))
			.accountsStrict({
				systemProgram: SystemProgram.programId,
				user: user.publicKey,
				vault,
				vaultState,
			})
      .signers([user])
			.rpc();
		console.log("✅ Your transaction signature:", txHelper(tx));

    await sleep(5);

		const vaultBalance = await connection.getBalance(vault);

		const userBalance = await connection.getBalance(user.publicKey);

    console.log("vault balance:", vaultBalance)
    console.log("user balance:", userBalance)
    console.log("before user balance:", beforeUserBalance)

		assert(vaultBalance === 0.2 * LAMPORTS_PER_SOL);
		assert(userBalance === beforeUserBalance - 0.2 * LAMPORTS_PER_SOL);
	});

	it("withdraws from vault", async () => {
		const beforeUserBalance = await connection.getBalance(user.publicKey);
		// Add your test here.
		const tx = await program.methods
			.withdraw(new anchor.BN(0.1 * LAMPORTS_PER_SOL))
			.accountsStrict({
				systemProgram: SystemProgram.programId,
				user: user.publicKey,
				vault,
				vaultState,
			})
			.signers([user])
			.rpc();
		console.log("✅ Your transaction signature:", txHelper(tx));

    await sleep(5);

		const vaultBalance = await connection.getBalance(vault);

		const userBalance = await connection.getBalance(user.publicKey);

    console.log("vault balance:", vaultBalance);
		console.log("user balance:", userBalance);
		console.log("before user balance:", beforeUserBalance);

		assert(vaultBalance === 0.1 * LAMPORTS_PER_SOL);
		assert(userBalance === beforeUserBalance + 0.1 * LAMPORTS_PER_SOL);
	});

	it("closes vault", async () => {
		const beforeUserBalance = await connection.getBalance(user.publicKey);
		// Add your test here.
		const tx = await program.methods
			.close()
			.accountsStrict({
				systemProgram: SystemProgram.programId,
				user: user.publicKey,
				vault,
				vaultState,
			})
			.signers([user])
			.rpc();
		console.log("✅ Your transaction signature:", txHelper(tx));

    await sleep(5);

		const vaultBalance = await connection.getBalance(vault);

		const userBalance = await connection.getBalance(user.publicKey);

    console.log("vault balance:", vaultBalance);
		console.log("user balance:", userBalance);
		console.log("before user balance:", beforeUserBalance);

		assert(vaultBalance === 0 * LAMPORTS_PER_SOL);
		assert(userBalance === 1 * LAMPORTS_PER_SOL);
	});
});
