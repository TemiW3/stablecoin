#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;


pub mod state;
pub mod constants;
pub mod instructions;
use state::*;
use constants::*;
use instructions::*;
pub use errors::*;
mod errors;




declare_id!("4FYnSZBqu28PL8rhezVzz1MXKNPTPo5Grwavfr6Lgfb9");

#[program]
pub mod stablecoin {
    use super::*;

   pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        process_initialize_config(ctx)
   }

   pub fn update_config(ctx: Context<UpdateConfig>, minimum_health_factor: u64) -> Result<()> {
        process_update_config(ctx, minimum_health_factor)
   }

     pub fn deposit_collateral_and_mint_tokens(
            ctx: Context<DepositCollateralAndMintTokens>,
            amount_collateral: u64,
            amount_tokens: u64,
      ) -> Result<()> {
            proccess_deposit_collateral_and_mint_tokens(ctx, amount_collateral, amount_tokens)
     }
}
