use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

declare_id!("EqDQt3TLCLZnuVja6BabumZB6iiaUAvmPW7xpZ27EQez");

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, receive, ctx.bumps.escrow)?;
        ctx.accounts.deposit_to_vault(amount)?;
        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.transfer_to_maker()?;
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.withdraw_and_close()?;
        Ok(())
    }
}
