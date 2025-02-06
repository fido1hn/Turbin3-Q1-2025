use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

declare_id!("8NmUxrPJWzNRnWSzqA2T4FM7zPayr2kJxWLjYPY4e3aW");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
