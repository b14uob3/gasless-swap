use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod state;

pub use constants::*;
pub use errors::ErrorCode;
pub use instructions::*;
pub use state::*;

declare_id!("2RoFjWiiCtNjsvWf2GcgqGd1MqPPFbZ9aCRwiUQSLmet");

#[program]
pub mod gasless_swap {
    use super::*;

    pub fn vault_init(ctx: Context<VaultInit>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn vault_close(ctx: Context<VaultClose>) -> Result<()> {
        ctx.accounts.process()
    }

    pub fn vault_deposit(ctx: Context<VaultDeposit>, amount: u64) -> Result<()> {
        ctx.accounts.process(amount)
    }
}
