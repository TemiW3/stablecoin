use anchor_lang::prelude::*;
use crate::{
        Config, 
        SEED_CONFIG_ACCOUNT, 
        SEED_MINT_ACCOUNT, 
        MINT_DECIMALS, 
        LIQUIDATIOND_THRESHOLD, 
        LIQUIDATION_BONUS, 
        MINIMUM_HEALTH_FACTOR
    };
use anchor_spl::token_interface::{
    Mint, 
    Token2022};


pub fn process_initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
    *ctx.accounts.config_account = Config {
        authority: ctx.accounts.authority.key(),
        mint_account: ctx.accounts.mint_account.key(),
        liquidation_threshold: LIQUIDATIOND_THRESHOLD,
        liquidation_bonus: LIQUIDATION_BONUS,
        minimum_health_factor: MINIMUM_HEALTH_FACTOR,
        bump: ctx.bumps.config_account,
        bump_mint_account: ctx.bumps.mint_account,
    };

    Ok(())

}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [SEED_CONFIG_ACCOUNT],
        bump
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [SEED_MINT_ACCOUNT],
        bump,
        mint::decimals = MINT_DECIMALS,
        mint::authority = mint_account,
        mint::freeze_authority = mint_account,
        mint::token_program = token_program
    )]
    pub mint_account: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

}