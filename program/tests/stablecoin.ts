import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablecoin } from "../target/types/stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

describe("stablecoin", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.stablecoin as Program<Stablecoin>;

  const pythSolanareceiver = new PythSolanaReceiver({ connection, wallet });

  const SOL_PRICE_FEED_ID =
    "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";

  const solUsdPriceFeedAccount = pythSolanareceiver.getPriceFeedAccountAddress(
    0,
    SOL_PRICE_FEED_ID
  );

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()],
    program.programId
  );

  it("Is initialized!", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Your transaction signature for initialize config: ", tx);
  });

  it("Deposit collateral and mint tokens", async () => {
    const amountCollateral = 1_000_000_000;
    const amountTokens = 1_000_000_000;

    const tx = await program.methods
      .depositCollateralAndMintTokens(
        new anchor.BN(amountCollateral),
        new anchor.BN(amountTokens)
      )
      .accounts({
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log(
      "Your transaction signature for deposit collateral and mint tokens: ",
      tx
    );
  });

  it("Redeem collateral and burn tokens", async () => {
    const amountCollateral = 500_000_000;
    const amountToBurn = 500_000_000;

    const tx = await program.methods
      .redeemCollateralAndBurningTokens(
        new anchor.BN(amountToBurn),
        new anchor.BN(amountCollateral)
      )
      .accounts({
        priceUpdate: solUsdPriceFeedAccount,
      })
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log(
      "Your transaction signature for redeem collateral and burn tokens: ",
      tx
    );
  });

  it("Update Config", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(1000))
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Your transaction signature for update config: ", tx);
  });

  it("Liquidate", async () => {
    const amountToBurn = 500_000_000;
    const tx = await program.methods
      .liquidate(new anchor.BN(amountToBurn))
      .accounts({ collateralAccount, priceUpdate: solUsdPriceFeedAccount })
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Your transaction signature for liquidation: ", tx);
  });

  it("Update Config", async () => {
    const tx = await program.methods
      .updateConfig(new anchor.BN(1))
      .accounts({})
      .rpc({ skipPreflight: true, commitment: "confirmed" });

    console.log("Your transaction signature for update config: ", tx);
  });
});
