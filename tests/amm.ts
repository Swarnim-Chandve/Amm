import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
import { Keypair, PublicKey } from "@solana/web3.js";
import * as spl from "@solana/spl-token";

describe("Amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .Amm as Program<Amm>;
  const SEED = new anchor.BN(1);
  const fee = 30;

  const user = Keypair.generate();
  let userTokenX: PublicKey, userTokenY: PublicKey;

  let tokenMintX: PublicKey, tokenMintY: PublicKey;
  let tokenXVault: PublicKey, tokenYVault: PublicKey;
  let config: PublicKey, tokenMintLp: PublicKey, userLp: PublicKey;

  before(async () => {
    const airdropSig = await provider.connection.requestAirdrop(
      user.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);

    tokenMintX = await spl.createMint(
      provider.connection,
      user,
      user.publicKey,
      null,
      6
    );
    tokenMintY = await spl.createMint(
      provider.connection,
      user,
      user.publicKey,
      null,
      6
    );

    await spl.createAssociatedTokenAccount(
      provider.connection,
      user,
      tokenMintX,
      user.publicKey
    );
    await spl.createAssociatedTokenAccount(
      provider.connection,
      user,
      tokenMintY,
      user.publicKey
    );

    userTokenX = spl.getAssociatedTokenAddressSync(tokenMintX, user.publicKey);
    userTokenY = spl.getAssociatedTokenAddressSync(tokenMintY, user.publicKey);

    await spl.mintTo(
      provider.connection,
      user,
      tokenMintX,
      userTokenX,
      user,
      200000000000
    );
    await spl.mintTo(
      provider.connection,
      user,
      tokenMintY,
      userTokenY,
      user,
      200000000000
    );

    config = PublicKey.findProgramAddressSync(
      [Buffer.from("config"), SEED.toArrayLike(Buffer, "le", 8)],
      program.programId
    )[0];

    tokenXVault = spl.getAssociatedTokenAddressSync(tokenMintX, config, true);
    tokenYVault = spl.getAssociatedTokenAddressSync(tokenMintY, config, true);

    tokenMintLp = PublicKey.findProgramAddressSync(
      [Buffer.from("lp"), config.toBuffer()],
      program.programId
    )[0];

    userLp = spl.getAssociatedTokenAddressSync(tokenMintLp, user.publicKey);
  });

  it("Initialize config", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(SEED, null, fee)
      .accountsPartial({
        initializer: user.publicKey,
        tokenMintX,
        tokenMintY,
        config,
        tokenMintLp,
        tokenXVault,
        tokenYVault,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("✅ AMM initialized successfully");
  });

  it("Deposit token to the empty pool", async () => {
    const tx = await program.methods
      .deposit(
        new anchor.BN(2000000),
        new anchor.BN(1000000),
        new anchor.BN(1000000)
      )
      .accountsPartial({
        user: user.publicKey,
        tokenMintX,
        tokenMintY,
        tokenMintLp,
        config,
        tokenXVault,
        tokenYVault,
        userTokenX,
        userTokenY,
        userLp,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      console.log("Your transaction signature", tx);
    console.log(
      "token_x_vault balance should be 1000000",
      await provider.connection.getTokenAccountBalance(tokenXVault)
    );
    console.log(
      "token_y_vault balance should be 1000000",
      await provider.connection.getTokenAccountBalance(tokenYVault)
    );
    console.log(
      "user_lp balance should be 2000000",
      await provider.connection.getTokenAccountBalance(userLp)
    );
  });
  
  it("swap token_x for token_y", async () => {
    const tx = await program.methods.swap(true, new anchor.BN(2345)).accountsPartial({
      trader: user.publicKey,
      tokenMintX,
      tokenMintY,
      tokenXVault,
      tokenYVault,
      config,
      traderTokenX: userTokenX,
      traderTokenY: userTokenY,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    //await provider.connection.confirmTransaction(tx);
    console.log("Your transaction signature", tx);
    console.log(
      "token_x_vault balance should be ---->",
      await provider.connection.getTokenAccountBalance(tokenXVault)
    );
    console.log(
      "token_Y_vault balance should be ---->",
      await provider.connection.getTokenAccountBalance(tokenYVault)
    );
    console.log(
      "user_token_x balance should be ----->",
      await provider.connection.getTokenAccountBalance(userTokenX)
    );
  })

  it("withdraw mint_token from lp_token and burn lp_token", async () => {
    const tx = await program.methods.withdraw(new anchor.BN(2000000),).accountsPartial({
      user: user.publicKey,
      tokenMintX,
      tokenMintY,
      tokenMintLp,
      config,
      tokenXVault,
      tokenYVault,
      userTokenX,
      userTokenY,
      userLp,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([user]).rpc();
    //await provider.connection.confirmTransaction(tx);
    console.log("Your transaction signature", tx);
    console.log(
      "token_x_vault balance should be ---->",
      await provider.connection.getTokenAccountBalance(tokenXVault)
    );
    console.log(
      "token_Y_vault balance should be ---->",
      await provider.connection.getTokenAccountBalance(tokenYVault)
    );
    console.log(
      "user_token_x balance should be ----->",
      await provider.connection.getTokenAccountBalance(userTokenX)
    );
  })
});