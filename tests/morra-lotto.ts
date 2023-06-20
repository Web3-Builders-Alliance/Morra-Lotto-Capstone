import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MorraLotto } from "../target/types/morra_lotto";
import { blake3 } from "hash-wasm";
import { PublicKey } from "@solana/web3.js";

describe("morra-lotto", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MorraLotto as Program<MorraLotto>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Create a blake3 hash", async () => {
    let key = anchor.web3.Keypair.generate();
    let hash = await createHash(1, 200, key.publicKey);
    console.log(hash);
  });
});

function createHash(hand: number, guess: number, secret: PublicKey) {
  const bufArray: Array<Buffer> = [];
  bufArray.push(writeUInt8(1));
  bufArray.push(writeUInt16(200));
  bufArray.push(secret.toBuffer());
  let buf = Buffer.concat(bufArray);
  return blake3(buf);
}

function writeUInt8(number: number) {
  const buf = Buffer.alloc(1);
  buf.writeUInt8(number);
  return buf;
}

function writeUInt16(number: number) {
  const buf = Buffer.alloc(2);
  buf.writeUInt8(number);
  return buf;
}
