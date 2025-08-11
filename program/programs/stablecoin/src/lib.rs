use anchor_lang::prelude::*;

declare_id!("4FYnSZBqu28PL8rhezVzz1MXKNPTPo5Grwavfr6Lgfb9");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
