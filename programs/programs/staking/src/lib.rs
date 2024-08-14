use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

declare_id!("EqDQt3TLCLZnuVja6BabumZB6iiaUAvmPW7xpZ27EQez");

#[program]
pub mod stake {
    use super::*;

    pub fn init_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn init_user_account(ctx: Context<InitUserAccount>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
