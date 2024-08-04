import fs from "fs"
import path from "path"
import os from "os"
import { Keypair } from "@solana/web3.js"

const filePath = path.join(os.homedir(), ".config", "solana", "id.json")

const walletString = fs.readFileSync(filePath, "utf8")

export const wallet = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(walletString)))
