use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        npl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::state::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,
    pub metadata: Account<'info, MetadataAccount>,
    pub edition: Account<'info, MasterEditionAccount>,
    #[account(
      mut,
      seeds = [b"user", user.key().as_ref()],
      bump = user_account.bump,
    )]
    pub config: Account<'info, StakeConfig>,
    pub user_account: Account<'info, UserAccount>,
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        Ok(())
    }
}
