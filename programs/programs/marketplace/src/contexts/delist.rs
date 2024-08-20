use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Delist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        seeds=[b"marketplace", name.as_str().as_bytes()],
        bump
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,

    pub maker_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    pub maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        close = maker,
        seeds=[marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        associated_token::mint=maker_mint,
        associated_token::authority=listing,
        associated_token::token_program=token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn withdraw_nft(&mut self) -> Result<()> {
        let signer_seeds = &[
            b"stake",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        // close account

        Ok(())
    }
}
