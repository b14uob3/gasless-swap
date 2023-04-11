use anchor_lang::prelude::*;

mod constants;
mod errors;
mod instructions;
mod state;

pub use constants::*;
pub use errors::*;
pub use instructions::*;
pub use state::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod gasless_swap {
    use super::*;

    pub fn vault_init(ctx: Context<VaultInit>) -> Result<()> {
        vault_init_inner()
    }
}

#[derive(Accounts)]
pub struct Initialize {}
