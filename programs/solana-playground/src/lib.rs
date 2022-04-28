use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_playground {
    use anchor_lang::solana_program::log::sol_log_compute_units;
    use anchor_lang::solana_program::sysvar;
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let now = sysvar::clock::Clock::get().unwrap().unix_timestamp as u64;
        msg!("Now = {:?}", now);
        sol_log_compute_units();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
