use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
  pub points_per_stake: u8,
  pub max_stake: u8,
  pub freeze_period: u32,
  pub rewards_bump: u8,
  pub bump: u8,
}


#[account]
#[derive(InitSpace)]
pub struct UserAccount {
  pub points: u32,
  pub amount_staked: u8,
  pub bump: u8,
}


#[account]
#[derive(InitSpace)]
pub struct StakeAccount{
  pub mint: Pubkey,
  pub owner: Pubkey,
  pub last_updated: i64,
  pub bump: u8,
}