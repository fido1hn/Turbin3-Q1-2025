use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

pub mod errors;

declare_id!("4UNeQ86dY6CRSMJvarSA4UTnjJQCxsUbDnnMVp3XAD7f");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, claim_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(claim_amount, max_x, max_y)?;
        Ok(())
    }
}
