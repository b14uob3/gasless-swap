use crate::*;

#[derive(Accounts)]
pub struct VaultDeposit<'info> {
    #[account(
        mut,
        seeds = [b"vault".as_ref()],
        bump,
    )]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> VaultDeposit<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        require!(
            self.vault.authority == *self.signer.key,
            ErrorCode::Unauthorized
        );

        // transfer sol from signer to vault
        anchor_lang::system_program::transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: self.signer.to_account_info(),
                    to: self.vault.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}
