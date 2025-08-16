# Stablecoin (Solana Anchor Program)

A collateralized stablecoin protocol built with Anchor on Solana. Users deposit SOL as collateral to mint a token-2022 stablecoin, redeem collateral by burning tokens, and can be liquidated if their position becomes undercollateralized. Pricing is powered by Pyth via the `pyth-solana-receiver` PriceUpdateV2 account.

> Status: reference/educational implementation. Not audited. Use at your own risk.

---

## Highlights

- Token-2022 mint with 9 decimals and PDA-controlled mint authority
- Per-user collateral vault PDA holding SOL
- Real-time pricing via Pyth `PriceUpdateV2`
- Health factor enforcement and liquidation flow
- Anchor 0.31.x program with TypeScript tests

---

## Table of Contents

- Overview
- Architecture
  - PDAs & Seeds
  - Accounts
- Instructions
- Health Factor & Pricing
- Building & Testing (localnet)
- Deploying
- TypeScript Client Examples
- Constants & Configuration
- Errors
- Repository Structure
- Known Limitations & Notes
- License

---

## Overview

- Users deposit SOL as collateral and mint a USD-denominated token (Token-2022) against that collateral.
- The protocol enforces a minimum health factor. If a position drops below the threshold, any liquidator can burn their tokens to seize a share of the collateral plus a liquidation bonus.

Program ID (localnet, as configured):

```
4FYnSZBqu28PL8rhezVzz1MXKNPTPo5Grwavfr6Lgfb9
```

See `Anchor.toml` for localnet configuration.

---

## Architecture

### PDAs & Seeds

- Config account PDA: seeds = `b"config"`
- Mint account PDA (Token-2022): seeds = `b"mint"`
- User collateral account PDA: seeds = `b"collateral", user_pubkey`
- User SOL vault PDA (SystemAccount): seeds = `b"sol", user_pubkey`

### Accounts

- `Config` (`programs/stablecoin/src/state.rs`)
  - `authority: Pubkey` — intended owner (not currently enforced for all admin ops; see notes)
  - `mint_account: Pubkey` — Token-2022 mint PDA
  - `liquidation_threshold: u64` — percent (e.g., 50 → 50%)
  - `liquidation_bonus: u64` — percent given to liquidator (e.g., 10 → 10%)
  - `minimum_health_factor: u64` — HF floor
  - `bump: u8` — config PDA bump
  - `bump_mint_account: u8` — mint PDA bump

- `Collateral` (per-user)
  - `depositor: Pubkey`
  - `sol_account: Pubkey` — user’s SOL vault PDA
  - `token_account: Pubkey` — user’s Token-2022 ATA
  - `lamport_balance: u64` — tracked SOL in vault
  - `amount_minted: u64` — user debt (stablecoin minted)
  - `bump, bump_sol_account: u8`
  - `is_initialized: bool`

---

## Instructions

All instruction handlers live under `programs/stablecoin/src/instructions`.

### initialize_config()

- Purpose: create config and mint PDAs with defaults; set mint authority to the mint PDA itself (PDA signer).
- Accounts:
  - `authority: Signer`
  - `config_account: Account<Config>` (PDA, init)
  - `mint_account: InterfaceAccount<Mint>` (PDA, init; Token-2022)
  - `token_program: Program<Token2022>`
  - `system_program: Program<System>`

### update_config(minimum_health_factor: u64)

- Purpose: update only `minimum_health_factor` on the config.
- Accounts:
  - `config_account: Account<Config>` (PDA)
- Note: Current implementation does not require an authority signer. Anyone can call this on localnet. See Known Limitations.

### deposit_collateral_and_mint_tokens(amount_collateral: u64, amount_tokens: u64)

- Purpose: deposit SOL into the user’s SOL vault PDA and mint Token-2022 stablecoins to the user’s ATA. Enforces health factor after applying the changes.
- Accounts:
  - `depositor: Signer`
  - `config_account: Account<Config>` (PDA; has_one mint_account)
  - `mint_account: InterfaceAccount<Mint>` (Token-2022 mint PDA)
  - `collateral_account: Account<Collateral>` (PDA, init_if_needed)
  - `sol_account: SystemAccount` (PDA; see Known Limitations)
  - `token_account: InterfaceAccount<TokenAccount>` (ATA, init_if_needed)
  - `system_program: Program<System>`
  - `token_program: Program<Token2022>`
  - `associated_token_program: Program<AssociatedToken>`
  - `price_update: Account<PriceUpdateV2>` (Pyth)

### redeem_collateral_and_burning_tokens(amount_to_burn: u64, amount_collateral: u64)

- Purpose: burn user tokens and withdraw SOL from the user’s SOL vault PDA back to the user. Enforces health factor after applying the changes.
- Accounts:
  - `depositor: Signer`
  - `price_update: Account<PriceUpdateV2>`
  - `config_account: Account<Config>` (PDA; has_one mint_account)
  - `collateral_account: Account<Collateral>` (PDA; has_one sol_account, token_account)
  - `sol_account: SystemAccount` (PDA)
  - `mint_account: InterfaceAccount<Mint>`
  - `token_account: InterfaceAccount<TokenAccount>` (user ATA)
  - `token_program: Program<Token2022>`
  - `system_program: Program<System>`

### liquidate(amount_to_burn: u64)

- Purpose: if a position’s HF < minimum, a liquidator can burn their tokens to seize a portion of the user’s SOL collateral plus a liquidation bonus.
- Accounts:
  - `liquidator: Signer`
  - `price_update: Account<PriceUpdateV2>`
  - `config_account: Account<Config>` (PDA; has_one mint_account)
  - `collateral_account: Account<Collateral>` (has_one sol_account)
  - `sol_account: SystemAccount` (borrower’s PDA)
  - `mint_account: InterfaceAccount<Mint>`
  - `token_account: InterfaceAccount<TokenAccount>` (liquidator’s ATA)
  - `token_program: Program<Token2022>`
  - `system_program: Program<System>`

---

## Health Factor & Pricing

Let:
- `collateral_lamports` = SOL in user’s vault PDA
- `USD(collateral)` = price in USD fetched from Pyth for SOL
- `debt` = `amount_minted`
- `lt` = `liquidation_threshold` percent (e.g., 50)

Then the health factor is:

```
HF = (USD(collateral) * lt / 100) / debt
```

- If `debt == 0`, HF is treated as `u64::MAX` (infinite).
- Deposit and redeem enforce `HF >= minimum_health_factor`.
- Liquidation requires `HF < minimum_health_factor`.

Pricing:
- Pulls from Pyth via `PriceUpdateV2.get_price_no_older_than(MAX_AGE, FEED_ID)`
- `FEED_ID` currently set to the SOL/USD feed in `constants.rs`.
- `PRICE_FEED_DECIMAL_ADJUSTMENT` is applied when converting lamports <-> USD.

---

## Building & Testing (localnet)

### Prerequisites

- Rust toolchain (stable)
- Solana CLI ≥ 1.17
- Anchor CLI matching `anchor-lang = 0.31.x`
- Node.js 18+ and Yarn

### Install toolchains (example)

```bash
# Solana
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor via avm (Anchor Version Manager)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# Verify
solana --version
anchor --version
```

### Install JS deps

```bash
cd program
yarn install
```

### Build

```bash
anchor build
```

### Test

```bash
# Spins up a local validator with config from Anchor.toml,
# clones required on-chain accounts (see [[test.validator.clone]]),
# then runs ts-mocha tests under tests/.
anchor test
```

Tests are in `tests/stablecoin.ts` and demonstrate all instructions using `@coral-xyz/anchor` and `@pythnetwork/pyth-solana-receiver`.

---

## Deploying

1. Set your provider in `Anchor.toml`:
   - `[provider] cluster = "devnet"` (or your target)
   - `wallet = "~/.config/solana/id.json"`
2. Ensure the `declare_id!` in `programs/stablecoin/src/lib.rs` matches the program address you intend to deploy (or update it with `anchor keys set` / `anchor build` flow).
3. Build and deploy:

```bash
anchor build
anchor deploy
```

Note: In `Anchor.toml` `[test]` sets `upgradeable = false` for tests. Adjust for your deployment policy if needed.

---

## TypeScript Client Examples

Derive PDAs:

```ts
import * as anchor from "@coral-xyz/anchor";

const program = anchor.workspace.stablecoin as anchor.Program<any>;
const wallet = (anchor.AnchorProvider.env().wallet as anchor.Wallet).publicKey;

const [configPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("config")],
  program.programId
);

const [mintPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("mint")],
  program.programId
);

const [collateralPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("collateral"), wallet.toBuffer()],
  program.programId
);

const [solVaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("sol"), wallet.toBuffer()],
  program.programId
);
```

Get the SOL/USD price update account via Pyth Solana Receiver:

```ts
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";

const provider = anchor.AnchorProvider.env();
const receiver = new PythSolanaReceiver({ connection: provider.connection, wallet: provider.wallet as any });
const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
const priceUpdate = receiver.getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID);
```

Initialize config:

```ts
await program.methods
  .initializeConfig()
  .accounts({})
  .rpc();
```

Deposit and mint:

```ts
await program.methods
  .depositCollateralAndMintTokens(new anchor.BN(1_000_000_000), new anchor.BN(1_000_000_000))
  .accounts({ priceUpdate })
  .rpc();
```

Redeem and burn:

```ts
await program.methods
  .redeemCollateralAndBurningTokens(new anchor.BN(500_000_000), new anchor.BN(500_000_000))
  .accounts({ priceUpdate })
  .rpc();
```

Liquidate:

```ts
await program.methods
  .liquidate(new anchor.BN(500_000_000))
  .accounts({ collateralAccount: collateralPda, priceUpdate })
  .rpc();
```

---

## Constants & Configuration

See `programs/stablecoin/src/constants.rs`:

- `MINT_DECIMALS: 9`
- `LIQUIDATIOND_THRESHOLD: 50` (typo in name, value used as 50%)
- `LIQUIDATION_BONUS: 10` (10%)
- `MINIMUM_HEALTH_FACTOR: 1`
- `FEED_ID: <SOL/USD feed>`
- `MAX_AGE: 100`
- `PRICE_FEED_DECIMAL_ADJUSTMENT: 10`

Localnet program mapping is set in `Anchor.toml` under `[programs.localnet]`.

The test validator clones required on-chain accounts defined in `[[test.validator.clone]]`. This enables Pyth access during `anchor test` without additional setup.

---

## Errors

From `programs/stablecoin/src/errors.rs`:

- `InvalidPrice` — Pyth price invalid or stale
- `BelowMinHealthFactor` — operation would push HF below minimum
- `AboveMinimumHealthFactor` — liquidation attempted on a healthy position

---

## Repository Structure

```
program/
  Anchor.toml               # localnet/test config
  Cargo.toml                # workspace
  package.json              # TS deps and linting
  programs/
    stablecoin/
      Cargo.toml
      src/
        lib.rs              # declare_id! and instruction entrypoints
        state.rs            # Config and Collateral accounts
        constants.rs        # seeds, thresholds, feed id
        errors.rs
        instructions/
          admin/
            initialize_config.rs
            update_config.rs
          deposit/
            deposit_collateral_and_mint_tokens.rs
            utils.rs
          withdraw/
            redeem_collateral_and_burning_tokens.rs
            liquidate.rs
            utils.rs
  tests/
    stablecoin.ts          # end-to-end examples
```

---

## Known Limitations & Notes

- Authority checks:
  - `update_config` does not currently require the `authority` signer. Anyone can call it on localnet. For production, gate this by authority.
- SOL vault initialization:
  - `sol_account` (user SOL vault PDA) is referenced in `deposit_collateral_and_mint_tokens` but is not `init`-ed there. Ensure it exists before first use or modify the program to `init_if_needed` for `sol_account`.
- Single collateral type:
  - Only SOL is supported as collateral. No multi-asset collateralization.
- Economics:
  - No interest, stability fees, or peg mechanisms are implemented; this is a minimal collateral/debt model.
- Token-2022:
  - The mint uses Token-2022. Ensure client ATAs are created with the Token-2022 associated token program.

---

## License

ISC (see `package.json`). If you need a Rust crate license file, add a top-level `LICENSE`.