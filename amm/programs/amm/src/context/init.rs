use anchor_lang::prelude::*;
use anchor_spl::token_interface::{TokenInterface, Mint};

use crate::state::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Box<InterfaceAccount<'info, Mint>>,
    pub mint_y: Box<InterfaceAccount<'info, Mint>>,

    #[account(
      init,
      payer = initializer,
      seeds = [b"config", seed.to_le_bytes().as_ref()],
      bump,
      space = Config::INIT_SPACE
    )]
    pub config: Box<Account<'info, Config>>,

    #[account(
      init, 
      payer = initializer,
      seeds = [b"lp", config.key().as_ref()],
      bump,
      mint::decimals = 6,
      mint::authority = config,
    )]
    pub mint_lp: Box<InterfaceAccount<'info, Mint>>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
  pub fn init(&mut self, seed:u64, fee: u16, authority: Option<Pubkey>, bumps: InitializeBumps) -> Result<()> {
    self.config.set_inner(Config { 
      seed, 
      authority, 
      mint_x: self.mint_x.key(), 
      mint_y: self.mint_y.key(), 
      fee, 
      locked: false, 
      config_bump: bumps.config, 
      lp_bump: bumps.mint_lp 
    });
    Ok(())
  }
}