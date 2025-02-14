use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

pub mod errors;

declare_id!("8NmUxrPJWzNRnWSzqA2T4FM7zPayr2kJxWLjYPY4e3aW");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init(points_per_stake, max_stake, freeze_period, ctx.bumps)?;
        Ok(())
    }

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.register_user(ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
}
