use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token_interface::{approve, Approve, Mint, TokenAccount, TokenInterface},
};

use crate::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub collection: InterfaceAccount<'info, Mint>,

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
        constraint=metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
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
        init_if_needed,
        payer=user,
        space=8+StakeAccount::INIT_SPACE,
        seeds=[b"stake".as_ref(), mint.key().as_ref(), config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account()]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub metadata_program: Program<'info, Metadata>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = self.metadata_program.to_account_info();
        let mint = &self.mint.to_account_info();

        FreezeDelegatedAccountCpi::new(
            &metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                edition,
                mint,
                token_account,
                token_program,
            },
        )
        .invoke()?;

        self.stake_account.set_inner(StakeAccount {
            bump: bumps.stake_account,
            last_updated: Clock::get()?.unix_timestamp,
            mint: mint.key(),
            owner: self.user.key(),
        });

        require!(
            self.user_account.amount_staked < self.config.max_stake,
            ErrorCode::MaxStake
        );

        self.user_account.amount_staked += 1;

        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Max stake amount excedded")]
    MaxStake,
}
