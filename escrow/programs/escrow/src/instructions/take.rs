use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::state::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    #[account(
      mint::token_program = token_program,
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
      mint::token_program = token_program,
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
      init_if_needed,
      payer = taker,
      associated_token::mint = mint_a,
      associated_token::authority = maker,
      associated_token::token_program = token_program,
    )]
    pub maker_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      associated_token::mint = mint_b,
      associated_token::authority = taker,
    )]
    pub taker_b_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
      mut,
      close = taker,
      seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
      bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
      mut,
      associated_token::mint = escrow.mint_a,
      associated_token::authority = escrow,
      associated_token::token_program = token_program,
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    token_program: Interface<'info, TokenInterface>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn transfer_to_maker(&mut self) -> Result<()> {
        let accounts = TransferChecked {
            from: self.taker_b_ata.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_a_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, self.escrow.receive, self.mint_b.decimals)?;
        Ok(())
    }

    pub fn withdraw_and_close(&mut self) -> Result<()> {
        let seed_bytes = self.escrow.seed.to_le_bytes();
        let bump = [self.escrow.bump];
        let signer_seeds = &[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &seed_bytes.as_ref(),
            &bump,
        ];

        let signer_seeds = &[&signer_seeds[..]];

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_b_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)?;

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        close_account(ctx)?;

        Ok(())
    }
}
