#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use state::*;
use constants::*;
use instructions::*;

mod state;
mod constants;
mod instructions;


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
}
