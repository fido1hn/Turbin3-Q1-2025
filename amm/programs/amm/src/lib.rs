use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod context;
pub use context::*;

declare_id!("4UNeQ86dY6CRSMJvarSA4UTnjJQCxsUbDnnMVp3XAD7f");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
