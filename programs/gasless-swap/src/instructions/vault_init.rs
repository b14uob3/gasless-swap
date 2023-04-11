use crate::*;

#[derive(Accounts)]
pub struct VaultInit<'info> {
    #[account(
        init,
        seeds = [b"vault".as_ref()],
        bump,
        payer = authority,
        space = Vault::size()
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultInit<'info> {
    pub fn process(&self) -> Result<()> {
        self.vault.authority = *self.authority.key;
        self.vault.min_rent = self.system_program.rent.minimum_balance(Vault::size());
        Ok(())
    }
}
