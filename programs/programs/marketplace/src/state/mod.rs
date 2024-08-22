use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
  pub maker: Pubkey,
  pub mint: Pubkey,
  pub price: u64,
  pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
  pub admin: Pubkey,
  pub fee: u16,
  pub rewards_bump: u8,
  pub treasury_bump: u8,
  #[max_len(50)]
  pub name: String,
  pub bump: u8,
}

