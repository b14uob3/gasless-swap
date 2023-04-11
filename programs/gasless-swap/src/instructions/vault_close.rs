use crate::*;

#[derive(Accounts)]
pub struct VaultClose<'info> {
    #[account(
        mut,
        close = authority,
        seeds = [b"vault".as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultClose<'info> {
    pub fn process(&self) -> Result<()> {
        require!(
            self.vault.authority == *self.authority.key,
            ErrorCode::Unauthorized
        );
        
        Ok(())
    }
}
