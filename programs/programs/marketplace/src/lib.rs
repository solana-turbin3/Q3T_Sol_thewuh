use anchor_lang::prelude::*;

pub mod contexts;
pub use contexts::*;

pub mod state;
pub use state::*;

declare_id!("EqDQt3TLCLZnuVja6BabumZB6iiaUAvmPW7xpZ27EQez");

#[program]
pub mod marketplace {
    use super::*;

    pub fn make(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
