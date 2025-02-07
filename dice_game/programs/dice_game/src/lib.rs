use anchor_lang::prelude::*;

declare_id!("HkQca1BQwgjbKtdRQiEpXeqnwMMkdBKXzHHkH4cAnDkd");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
