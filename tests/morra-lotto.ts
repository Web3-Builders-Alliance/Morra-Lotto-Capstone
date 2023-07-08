import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MorraLotto } from "../target/types/morra_lotto";
import { blake3 } from "hash-wasm";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { wallet } from "./dev-wallet";
import bs58 from "bs58";
// import { hash } from "@coral-xyz/anchor/dist/cjs/utils/sha256";
import { createHash } from "crypto";
// import * as blake3 from "blake3";
import { hash } from "blake3";
import { BN } from "bn.js";

describe("morra-lotto", async () => {
  const gameSeed = new Keypair().publicKey;

  // follow repo to create a blake3 hash. provide that hash to intialize function
  let gamesMoves = await buildHash(1, 7, gameSeed);
  let hashAddress = new PublicKey(new BN(gamesMoves)); // Convert string to address
  let hashArray = convertStringToArray(gamesMoves);

  // let hashAddress = new PublicKey(
  //   gamesMoves,
  // );

  // const hash = blake3(32); // 32-byte (256-bit) hash
  // hash.update(gameSeed.toBuffer());
  // const digest = await hash.digest();

  // const gameHash = PublicKey.decode(hash.digest());

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
  // it("Create a blake3 hash", async () => {
  //   let key = anchor.web3.Keypair.generate();
  //   let hash = await buildHash(1, 7, key.publicKey);
  //   let hashAddress = new PublicKey(hash); // Convert string to address
  // });

  // create hash
  // it("Create a blake3 hash", async () => {
  //   let gamesMoves = await buildHash(1, 7, gameSeed);

  //   hashAddress = new PublicKey(gamesMoves).toBase58(); // Convert string to address

  //   console.log(gamesMoves);
  //   console.log(hashAddress);
  // });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(hashArray, new anchor.BN(1 * LAMPORTS_PER_SOL), 3, 7)
      .accounts({
        player: player.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth,
        vault,
        game: game.publicKey,
        hash: hashAddress,
      })
      .signers([player, vaultState])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Play!", async () => {
    const tx = await program.methods
      .play(4)
      .accounts({
        player: player.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth,
        vault,
        gameHash: hashAddress,
        game: game.publicKey,
      })
      .signers([player])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});

function buildHash(hand: number, guess: number, secret: PublicKey) {
  const bufArray: Array<Buffer> = [];
  bufArray.push(writeUInt8(hand));
  bufArray.push(writeUInt16(guess));
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

function convertStringToArray(str: string): number[] {
  const encoder = new TextEncoder();
  const encodedString = encoder.encode(str);
  const array = Array.from(encodedString.slice(0, 32));
  return array;
}
