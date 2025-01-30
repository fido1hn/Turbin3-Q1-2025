use anchor_lang::prelude::*;

pub mod context;
pub use context::*;

pub mod state;
pub use state::*;

pub mod error;
pub use error::*;

declare_id!("djGsFzDKkbkYPsSjuc6zFESs8bXRGiamvpWmwM7ANRk");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, ctx.bumps)?;
        Ok(())
    }

    pub fn listing(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, ctx.bumps)?;
        ctx.accounts.deposit_nft()?;
        Ok(())
    }
}
