use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, MintTo, mint_to},
};

use crate::{StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds=[b"config".as_ref()],
        bump=config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump=user_account.bump,
        
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds=[b"rewards", config.key().as_ref()],
        bump=config.rewards_bump,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=rewards_mint,
        associated_token::authority=user,
        associated_token::token_program=token_program
    )]
    pub rewards_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {

        let cpi_accounts = MintTo {
            authority: self.config.to_account_info(),
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
        };

        let cpi_program = self.token_program.to_account_info();

        let signer_seeds = &[
            b"config".as_ref(),
            &[self.config.bump],
        ];

        let signer = [&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer);

        mint_to(cpi_ctx, self.user_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32))?;

        self.user_account.points = 0;

        Ok(())
    }


}
