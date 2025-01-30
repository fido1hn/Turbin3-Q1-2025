use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name is too long")]
    NameTooLong,
}
