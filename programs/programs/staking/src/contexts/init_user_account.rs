use anchor_lang::prelude::*;

use crate::UserAccount;

#[derive(Accounts)]
pub struct InitUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer=user,
        space=8+UserAccount::INIT_SPACE,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitUserAccount<'info> {
    pub fn init_user_account(
        &mut self,
        bumps: &InitUserAccountBumps,
    ) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            bump: bumps.user_account,
            amount_staked: 0,
            points: 0,
        });
        Ok(())
    }
}
