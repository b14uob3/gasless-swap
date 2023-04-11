use crate::*;

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub min_rent: u64,
}

impl Vault {
    pub fn size() -> usize {
        DISCRIMINATOR_SIZE
        + PUBKEY_SIZE // authority
        + U64_SIZE // min_rent
    }
}
