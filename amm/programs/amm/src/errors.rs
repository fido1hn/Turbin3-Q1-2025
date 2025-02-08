use anchor_lang::error_code;

#[error_code]
pub enum AmmError {
    #[msg("Some Error Occured!")]
    SomeError,
}
