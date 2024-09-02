use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    solana_program::{
        ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
    },
    system_program::{transfer, Transfer},
};

declare_id!("43qCMvFpQR1DgE4AiNbjrQXqakoTnr7DrTw8zfGNNYJM");

pub const HOUSE_EDGE: u16 = 150;

#[program]
pub mod randomnmess {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    /// CHECK: house
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        has_one=player,
        seeds=[b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump=bet.bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address = anchor_lang::solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;

        require_keys_eq!(ix.program_id, ed25519_program::ID, ErrorCode::SomeError);

        require_eq!(ix.accounts.len(), 0, ErrorCode::SomeError);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, ErrorCode::SomeError);

        let signature = &signatures[0];

        require_keys_eq!(
            signature.public_key.ok_or(ErrorCode::SomeError)?,
            self.house.key(),
            ErrorCode::SomeError
        );

        require!(
            &signature.signature.ok_or(ErrorCode::SomeError)?.eq(sig),
            ErrorCode::SomeError
        );

        require!(
            &signature
                .message
                .as_ref()
                .ok_or(ErrorCode::SomeError)?
                .eq(&self.bet.to_slice()),
            ErrorCode::SomeError
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &ResolveBetBumps, sig: &[u8]) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payment = (self.bet.amount as u128)
                .checked_mul(10000 - HOUSE_EDGE as u128)
                .ok_or(ErrorCode::SomeError)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(ErrorCode::SomeError)?
                .checked_div(100)
                .ok_or(ErrorCode::SomeError)?;

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let signer_seeds = &[
                b"vault",
                self.house.to_account_info().key.as_ref(),
                &[bumps.vault],
            ];

            let signer = &[&signer_seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, signer);

            transfer(cpi_ctx, payment as u64)?
        }
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("some error")]
    SomeError,
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: house
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        init,
        payer=player,
        space=8+Bet::INIT_SPACE,
        seeds=[b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        seed: u64,
        amount: u64,
        roll: u8,
        bumps: &PlaceBetBumps,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            amount,
            bump: bumps.bet,
            player: self.player.key(),
            roll,
            seed,
            slot: Clock::get()?.slot,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}

// state
#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub seed: u64,
    pub amount: u64,
    pub roll: u8,
    pub slot: u64,
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.player.to_bytes().to_vec();
        s.extend_from_slice(&self.seed.to_le_bytes());
        s.extend_from_slice(&self.amount.to_le_bytes());
        s.extend_from_slice(&self.roll.to_le_bytes());
        s.extend_from_slice(&self.slot.to_le_bytes());
        s.extend_from_slice(&self.bump.to_le_bytes());
        s
    }
}
