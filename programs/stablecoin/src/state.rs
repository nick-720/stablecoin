use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    pub depositor: Pubkey,
    pub sol_account: Pubkey, // PDA of SOL collateral account
    pub token_account: Pubkey, // PDA of stablecoin account
    pub lamport_balance: u64,
    pub amount_minted: u64,
    pub bump: u8,
    pub bump_sol_account: u8,
    pub is_initialized: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct Config { // holds global state of stablecoin
    pub authority: Pubkey, //auth of overall stablecoin
    pub mint_account: Pubkey, // the mint for the stablecoin
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub min_health_factor: u64,
    pub bump: u8, //bump for config account
    pub bump_mint_account: u8, //bump for stablecoin mint account pda
}