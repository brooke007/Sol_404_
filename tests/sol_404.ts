import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sol404 } from "../target/types/sol_404";
import { amount, Metaplex } from "@metaplex-foundation/js"
import * as fs from 'fs';
import * as spl from "@solana/spl-token"
import { assert } from "chai"
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import * as web3 from '@solana/web3.js';
import { getMint, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Connection, clusterApiUrl } from '@solana/web3.js';



describe("sol_404", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sol404 as Program<Sol404>;

  const connection = new Connection("http://127.0.0.1:8899", "confirmed");
  const metaplex = Metaplex.make(connection)
  
  const privateKeyJson = "/.config/solana/id.json"
  const privateKeyString = fs.readFileSync(privateKeyJson, { encoding: 'utf8' });
  const privateKeyUint8Array = new Uint8Array(JSON.parse(privateKeyString));
  const admin = anchor.web3.Keypair.fromSecretKey(privateKeyUint8Array);
  const buyer = anchor.web3.Keypair.generate()

  connection.requestAirdrop(buyer.publicKey, 100*anchor.web3.LAMPORTS_PER_SOL)

  console.log("buyer address:", buyer.publicKey.toBase58())
  
  const [JellyTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("jellya")],
    program.programId
  )

  const [UsdcTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("usdc")],
    program.programId
  )

  const [NftTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("nft")],
    program.programId
  )

  const JellyTokenAccount = spl.getAssociatedTokenAddressSync(
    JellyTokenMintPDA,
    admin.publicKey
  )

  const UsdcTokenAccount = spl.getAssociatedTokenAddressSync(
    UsdcTokenMintPDA,
    admin.publicKey
  )

  const NftTokenAccount = spl.getAssociatedTokenAddressSync(
    NftTokenMintPDA,
    admin.publicKey
  )

  //buyer jelly token account
  const BuyerTokenAccount = spl.getAssociatedTokenAddressSync(
    JellyTokenMintPDA,
    buyer.publicKey
  )

  // buyer usdc token account
  const BuyerUsdcAccount = spl.getAssociatedTokenAddressSync(
    UsdcTokenMintPDA,
    buyer.publicKey
  )

  const BuyerNftAccount = spl.getAssociatedTokenAddressSync(
    NftTokenMintPDA,
    buyer.publicKey
  )

  const jellymetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "Jelly",
    symbol: "jelly",
  }

  const usdcmetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "usdc",
    symbol: "usdc",
  }

  const nftmetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "nft",
    symbol: "nft",
  }

  it("Is create mint!", async () => {
    const jellyTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: JellyTokenMintPDA });

    const usdcTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: UsdcTokenMintPDA });

    const nftTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: NftTokenMintPDA });

    const tx = await program.methods
    .createmint(jellymetadata.uri, usdcmetadata.uri, nftmetadata.uri, jellymetadata.name, usdcmetadata.name, nftmetadata.name, jellymetadata.symbol, usdcmetadata.symbol, nftmetadata.symbol)
    .accounts({
      admin: admin.publicKey,
      jellyTokenMint: JellyTokenMintPDA,
      usdcTokenMint: UsdcTokenMintPDA,
      nftMint: NftTokenMintPDA,
      jellyMetadataAccount: jellyTokenMintMetadataPDA,
      usdcMetadataAccount: usdcTokenMintMetadataPDA,
      nftMetadataAccount: nftTokenMintMetadataPDA,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Is transfer usdc tokens!", async() => {
    const tx = await program.methods
    .transfer()
    .accounts({
      buyer: buyer.publicKey,
      usdcTokenMint: UsdcTokenMintPDA,
      buyerUsdcAccount: BuyerUsdcAccount,
      
    })
    .signers([buyer])
    .rpc();
    console.log("Your transaction signature", tx);
  })

  const usdc_amount = new anchor.BN(13);
  it("Is buy jelly tokens!", async() => {
    const tx = await program.methods
    .buytokens(usdc_amount)
    .accounts({
      buyer: buyer.publicKey,
      admin: admin.publicKey,
      buyerTokenAccount: BuyerTokenAccount,
      buyerUsdcAccount: BuyerUsdcAccount,
      buyerNftAccount: BuyerNftAccount,
      jellyTokenMint: JellyTokenMintPDA,
      nftMint: NftTokenMintPDA,
      usdcTokenMint: UsdcTokenMintPDA,
      jellyTokenAccount: JellyTokenAccount,
      nftTokenAccount: NftTokenAccount,
      usdcTokenAccount: UsdcTokenAccount
    })
    .signers([buyer, admin])
    .rpc().catch(e => console.error(e));
    console.log("Your transaction signature", tx);
  })

  it("Is sold jelly tokens!", async() => {
    const tx = await program.methods
    .usetokens(usdc_amount)
    .accounts({
      buyer: buyer.publicKey,
      admin: admin.publicKey,
      buyerTokenAccount: BuyerTokenAccount,
      buyerUsdcAccount: BuyerUsdcAccount,
      buyerNftAccount: BuyerNftAccount,
      jellyTokenMint: JellyTokenMintPDA,
      nftMint: NftTokenMintPDA,
      usdcTokenMint: UsdcTokenMintPDA,
      jellyTokenAccount: JellyTokenAccount,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .signers([buyer, admin])
    .rpc().catch(e => console.error(e));
    console.log("Your transaction signature", tx);
  })
});
