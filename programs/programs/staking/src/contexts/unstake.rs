use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{revoke, Mint, Revoke, TokenAccount, TokenInterface},
};

use crate::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user,
    )]
    pub mint_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds= [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint=metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds= [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account()]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump=user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds=[b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump=stake_account.bump,
        close=user,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account()]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub metadata_program: Program<'info, Metadata>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = self.metadata_program.to_account_info();
        let mint = &self.mint.to_account_info();

        let cpi_accounts = ThawDelegatedAccountCpiAccounts {
            delegate,
            edition,
            mint,
            token_account,
            token_program,
        };

        ThawDelegatedAccountCpi::new(&metadata_program, cpi_accounts).invoke()?;

        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Revoke {
            authority: self.stake_account.to_account_info(),
            source: self.mint_ata.to_account_info(),
        };

        let signer_seeds = &[
            b"stake".as_ref(),
            mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ];

        let signer = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        revoke(cpi_ctx)?;

        self.user_account.amount_staked -= 1;

        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.last_updated) / 86400) as u32;

        self.user_account.points += time_elapsed as u32 * self.config.points_per_stake as u32;

        Ok(())
    }
}
