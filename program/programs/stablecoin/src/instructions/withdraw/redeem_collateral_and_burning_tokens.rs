use anchor_lang::prelude::*;

use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;
use anchor_spl::token_interface::{TokenAccount, Token2022, Mint};
use crate::constants::{SEED_COLLATERAL_ACCOUNT, SEED_CONFIG_ACCOUNT};
use crate::instructions::{burn_tokens, withdraw_sol};
use crate::state::{Collateral, Config}; 
use crate::utils::check_health_factor;


pub fn process_redeem_collateral_and_burning_tokens(
    ctx: Context<RedeemCollateralAndBurningTokens>,
    amount_to_burn: u64,
    amount_collateral: u64,
) -> Result<()> {
    let collateral_account = &mut ctx.accounts.collateral_account;
    collateral_account.lamport_balance = ctx.accounts.sol_account.lamports() - amount_collateral;
    collateral_account.amount_minted -= amount_to_burn;


    check_health_factor(
        &ctx.accounts.collateral_account, 
        &ctx.accounts.config_account, 
        &ctx.accounts.price_update)?;

    burn_tokens(
        &ctx.accounts.token_program,
        &ctx.accounts.mint_account, 
        &ctx.accounts.token_account, 
        &ctx.accounts.redeemer, 
        amount_to_burn)?;

    withdraw_sol(
        ctx.accounts.collateral_account.bump_sol_account, 
        &ctx.accounts.redeemer.key(), 
        &ctx.accounts.system_program, 
        &ctx.accounts.sol_account, 
        &ctx.accounts.redeemer.to_account_info(), 
        amount_collateral)?;

    Ok(())
}

#[derive(Accounts)]
pub struct RedeemCollateralAndBurningTokens<'info> {
    #[account(mut)]
    pub redeemer: Signer <'info>,

    pub price_update: Account<'info, PriceUpdateV2>,

    #[account(
        seeds = [SEED_CONFIG_ACCOUNT],
        bump = config_account.bump,
        has_one = mint_account,
    )]
    pub config_account: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_COLLATERAL_ACCOUNT, redeemer.key().as_ref()],
        bump = collateral_account.bump,
        has_one = sol_account,
        has_one = token_account,
    )]
    pub collateral_account: Account<'info, Collateral>,

    #[account(mut)]
    pub sol_account: SystemAccount<'info>,
    #[account(mut)]
    pub mint_account: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,

}