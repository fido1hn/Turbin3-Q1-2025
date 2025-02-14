use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::{state::{StakeAccount, StakeConfig, UserAccount}, errors::StakeErrors};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = user,
        associated_token::mint = nft_mint,
    )]
    pub nft_mint_ata: Account<'info, TokenAccount>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        seeds = [b"metadata", 
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(), 
        constraint = metadata.collection.as_ref().unwrap().verified, 
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [b"metadata", 
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
      seeds = [b"stake_config"],
      bump = config.bump,
    )]
    pub config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer = user,
        space = StakeAccount::INIT_SPACE + 8,
        seeds = [b"stake_account", nft_mint.key().as_ref(), config.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, bumps: StakeBumps) -> Result<()> {
        require!(self.user_account.amount_staked < self.config.max_stake, StakeErrors::MaxStakeReached);

        self.user_account.amount_staked += 1;

        let clock = Clock::get()?;
        self.stake_account.set_inner(StakeAccount{
            owner: self.user.key(),
            nft_mint: self.nft_mint.key(),
            staked_at: clock.unix_timestamp,
            bump: bumps.stake_account
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Approve {
            to: self.nft_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        approve(cpi_ctx, 1)?;

        // stake account now has authority of nft

        let cpi_program = &self.metadata_program.to_account_info();
        let token_program = &self.token_program.to_account_info();


        let cpi_accounts = FreezeDelegatedAccountCpiAccounts { 
            delegate: &self.stake_account.to_account_info(), 
            token_account: &self.nft_mint_ata.to_account_info(), 
            edition: &self.edition.to_account_info(), 
            mint: &self.edition.to_account_info(), 
            token_program
        };

        let seeds = &[b"stake_account", 
            self.nft_mint.to_account_info().key.as_ref(), 
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump]];

        let signers_seeds = &[&seeds[..]];

        FreezeDelegatedAccountCpi::new(cpi_program, cpi_accounts).invoke_signed(signers_seeds)?;

        Ok(())
    }
}
