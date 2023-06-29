import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MorraLotto } from "../target/types/morra_lotto";
import { blake3 } from "hash-wasm";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { wallet } from "./dev-wallet";
import { hash } from "@coral-xyz/anchor/dist/cjs/utils/sha256";

describe("morra-lotto", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.MorraLotto as Program<MorraLotto>;

  // Set out wallet
  const player = Keypair.fromSecretKey(new Uint8Array(wallet));

  const vaultState = Keypair.generate();
  const game = Keypair.generate();

  const vaultAuth_seeds = [
    Buffer.from("auth"),
    vaultState.publicKey.toBuffer(),
  ];
  const vaultAuth = PublicKey.findProgramAddressSync(
    vaultAuth_seeds,
    program.programId
  )[0];

  const vault_seeds = [Buffer.from("vault"), vaultAuth.toBuffer()];
  const vault = PublicKey.findProgramAddressSync(
    vault_seeds,
    program.programId
  )[0];

  // create hash
  it("Create a blake3 hash", async () => {
    let key = anchor.web3.Keypair.generate();
    let hash = await createHash(1, 7, key.publicKey);
    console.log(hash);
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize({
        hash,
        hand: 4,
        guess: 8,
        betAmount: new anchor.BN(1 * LAMPORTS_PER_SOL),
      })
      .accounts({
        player: player.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth,
        vault,
        game: game.publicKey,
        hash: ,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

function createHash(hand: number, guess: number, secret: PublicKey) {
  const bufArray: Array<Buffer> = [];
  bufArray.push(writeUInt8(1));
  bufArray.push(writeUInt16(7));
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
