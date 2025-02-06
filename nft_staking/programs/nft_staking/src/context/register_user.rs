use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
  #[account(mut)]
  pub user: Signer<'info>,
  #[account(
    init, 
    payer = user, 
    space = 8 + UserAccount::INIT_SPACE,
    seeds = [b"user_account", user.key().as_ref()],
    bump
  )]
  pub user_account: Account<'info, UserAccount>,
  pub system_program: Program<'info, System>
}

impl<'info> RegisterUser<'info> {
  pub fn register_user(&mut self, bumps: RegisterUserBumps) -> Result<()> {
    self.user_account.set_inner(UserAccount { points: (), amount_staked: (), bump: () });
    Ok(())
  }
}