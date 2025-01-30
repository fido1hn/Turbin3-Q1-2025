use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{state::Marketplace, Listing};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
      seeds = [b"marketplace", marketplace.name.as_bytes()],
      bump = marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
      init_if_needed,
      payer = taker,
      associated_token::mint = maker_mint,
      associated_token::authority = taker
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      associated_token::mint = maker_mint,
      associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      seeds = [maker_mint.key().as_ref(), marketplace.key().as_ref()],
      bump = listing.bump,
      close = maker
    )]
    pub listing: Account<'info, Listing>,
    #[account(
      mut,
      seeds = [b"treasury", marketplace.key().as_ref()],
      bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
      mut,
      seeds = [b"rewards", marketplace.key().as_ref()],
      bump = marketplace.rewards_mint_bump,
      mint::authority = marketplace,
      mint::decimals = 6,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self) -> Result<()> {
        Ok(())
    }
}
