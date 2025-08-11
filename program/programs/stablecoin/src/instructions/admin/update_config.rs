use anchor_lang::prelude::*;
use crate::{
    state::Config,
    constants::{SEED_CONFIG_ACCOUNT},
};

pub fn process_update_config(ctx: Context<UpdateConfig>, minimum_health_factor: u64) -> Result <()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.minimum_health_factor = minimum_health_factor;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump
    )]
    pub config_account: Account<'info, Config>,
}