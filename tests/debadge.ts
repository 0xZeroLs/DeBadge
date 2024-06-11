import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Debadge } from "../target/types/debadge";

import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";

const RPC_URL = "http://127.0.0.1:8899";
const DECIMALS_PER_TOKEN = 100000;

const createTokenMint = async (
  connection: any,
  payer: anchor.Wallet,
  mintKeypair: anchor.web3.Keypair
) => {
  try {
    const mint = await createMint(
      connection,
      payer.payer,
      payer.publicKey,
      payer.publicKey,
      5,
      mintKeypair
    );

    // console.log(mint);
  } catch (e) {
    console.log(e);
  }
};

// const getKey = anchor.web3.Keypair.generate();
// console.log(getKey);

describe("debadge", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const payer = provider.wallet as anchor.Wallet;
  const connection = new Connection(RPC_URL, "confirmed");
  const program = anchor.workspace.Debadge as Program<Debadge>;

  const mintAndSendTokens = async (
    mintPubKey: PublicKey,
    destination: PublicKey,
    amount: number
  ) => {
    await mintTo(
      connection,
      payer.payer,
      mintPubKey,
      destination,
      payer.payer,
      amount * DECIMALS_PER_TOKEN
    );
  };

  // 44WAtPrJrbh9a7cw2PesBAvntf5xiqgKtmoBLT4CjMVL
  const bonkToken = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array([
      180, 119, 104, 115, 201, 223, 73, 68, 77, 98, 180, 75, 39, 237, 166, 65,
      179, 36, 98, 31, 188, 112, 64, 183, 33, 11, 242, 246, 105, 201, 165, 143,
      45, 120, 201, 77, 136, 97, 47, 49, 116, 245, 172, 76, 125, 61, 192, 129,
      245, 14, 0, 183, 60, 197, 220, 180, 211, 81, 121, 119, 251, 153, 16, 91,
    ])
  );

  const [TREASURY_VAULT_SEED] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("treasury_vault")],
    program.programId
  );

  const [PLATFORM_INFO_SEED] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("platform_info")],
    program.programId
  );

  const [BADGE_SEED] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("badge"),
      payer.publicKey.toBuffer(),
      Buffer.from("Test Badge"),
    ],
    program.programId
  );

  const [BADGE_VAULT_SEED] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("badge_vault"), BADGE_SEED.toBuffer()],
    program.programId
  );

  const [USER_BADGE_ACCOUNT_SEED] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("badge"), BADGE_SEED.toBuffer(), payer.publicKey.toBuffer()],
      program.programId
    );

  it("Is initialized!", async () => {
    await createTokenMint(connection, payer, bonkToken);

    await program.methods
      .initialize()
      .accounts({
        platformInfo: PLATFORM_INFO_SEED,
        treasuryVault: TREASURY_VAULT_SEED,
        user: payer.publicKey,
        feeTokenMint: bonkToken.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        console.log(err);
      });

    const platformInfo = await program.account.platformInfo.fetch(
      PLATFORM_INFO_SEED
    );

    console.log(platformInfo);
  });

  it("Creates a badge", async () => {
    let max_supply = new anchor.BN(1000000 * DECIMALS_PER_TOKEN);
    let decimals: any = new anchor.BN(5);
    let name = "Test Badge";
    let symbol = "TB";

    const user_token_account = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      bonkToken.publicKey,
      payer.publicKey
    );

    await mintAndSendTokens(
      bonkToken.publicKey,
      user_token_account.address,
      100
    );

    await new Promise((resolve) => setTimeout(resolve, 15000));
    console.log("Token Balance", user_token_account.amount.toString());

    await program.methods
      .createBadge(max_supply, decimals, name, symbol)
      .accounts({
        platformInfo: PLATFORM_INFO_SEED,
        treasuryVault: TREASURY_VAULT_SEED,
        badge: BADGE_SEED,
        badgeVault: BADGE_VAULT_SEED,
        userTokenAccount: user_token_account.address,
        user: payer.publicKey,
        feeTokenMint: bonkToken.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        console.log(err);
      });

    const badge = await program.account.badge.fetch(BADGE_SEED);
    console.log(badge);

    const platformInfo = await program.account.platformInfo.fetch(
      PLATFORM_INFO_SEED
    );

    console.log(platformInfo);
  });

  it("Mints badges", async () => {
    const user_token_account = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      bonkToken.publicKey,
      payer.publicKey
    );

    let amountToBuy = new anchor.BN(100 * DECIMALS_PER_TOKEN);

    await program.methods
      .mintBadge(amountToBuy)
      .accounts({
        badge: BADGE_SEED,
        userBadgeAccount: USER_BADGE_ACCOUNT_SEED,
        treasuryVault: TREASURY_VAULT_SEED,
        badgeVault: BADGE_VAULT_SEED,
        userTokenAccount: user_token_account.address,
        user: payer.publicKey,
        feeTokenMint: bonkToken.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        console.log(err);
      });

    const badge = await program.account.badge.fetch(BADGE_SEED);

    // get user badge account
    const userBadgeAccount = await program.account.badgeAccount.fetch(
      USER_BADGE_ACCOUNT_SEED
    );

    console.log(badge.price.toString());
    console.log(userBadgeAccount.balance.toString());
  });

  it("Checks the user token account balance", async () => {
    await new Promise((resolve) => setTimeout(resolve, 15000));

    const user_token_account = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      bonkToken.publicKey,
      payer.publicKey
    );

    console.log("Token Balance", user_token_account.amount.toString());
  });

  // it("Mints more badges", async () => {
  //   const user_token_account = await getOrCreateAssociatedTokenAccount(
  //     connection,
  //     payer.payer,
  //     bonkToken.publicKey,
  //     payer.publicKey
  //   );

  //   let amountToBuy = new anchor.BN(100 * DECIMALS_PER_TOKEN);

  //   await program.methods
  //     .mintBadge(amountToBuy)
  //     .accounts({
  //       badge: BADGE_SEED,
  //       userBadgeAccount: USER_BADGE_ACCOUNT_SEED,
  //       treasuryVault: TREASURY_VAULT_SEED,
  //       badgeVault: BADGE_VAULT_SEED,
  //       userTokenAccount: user_token_account.address,
  //       user: payer.publicKey,
  //       feeTokenMint: bonkToken.publicKey,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       systemProgram: SystemProgram.programId,
  //     })
  //     .rpc()
  //     .then((res) => {
  //       console.log(res);
  //     })
  //     .catch((err) => {
  //       console.log(err);
  //     });

  //   const badge = await program.account.badge.fetch(BADGE_SEED);

  //   // get user badge account
  //   const userBadgeAccount = await program.account.badgeAccount.fetch(
  //     USER_BADGE_ACCOUNT_SEED
  //   );

  //   console.log(badge.price.toString());
  //   console.log(userBadgeAccount.balance.toString());
  // });

  it("Burns badges", async () => {
    const user_token_account = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      bonkToken.publicKey,
      payer.publicKey
    );

    let amountToBurn = new anchor.BN(100 * DECIMALS_PER_TOKEN);

    await program.methods
      .burnBadge(amountToBurn)
      .accounts({
        badge: BADGE_SEED,
        userBadgeAccount: USER_BADGE_ACCOUNT_SEED,
        treasuryVault: TREASURY_VAULT_SEED,
        badgeVault: BADGE_VAULT_SEED,
        userTokenAccount: user_token_account.address,
        user: payer.publicKey,
        feeTokenMint: bonkToken.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc()
      .then((res) => {
        console.log(res);
      })
      .catch((err) => {
        console.log(err);
      });

    const badge = await program.account.badge.fetch(BADGE_SEED);

    // get user badge account
    const userBadgeAccount = await program.account.badgeAccount.fetch(
      USER_BADGE_ACCOUNT_SEED
    );

    console.log(badge.price.toString());
    console.log(userBadgeAccount.balance.toString());
  });

  it("Checks the user token account balance", async () => {
    await new Promise((resolve) => setTimeout(resolve, 15000));

    const user_token_account = await getOrCreateAssociatedTokenAccount(
      connection,
      payer.payer,
      bonkToken.publicKey,
      payer.publicKey
    );

    console.log("Token Balance", user_token_account.amount.toString());
  });
});
