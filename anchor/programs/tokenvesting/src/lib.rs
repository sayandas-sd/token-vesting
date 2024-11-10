#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod tokenvesting {
    use super::*;

    pub fn creating_vesting_account(ctx: Contex<CreatingVestingAccount>, company_name: String) -> Result<()> {
      Ok(())
    }

  
}



#[derive(Acounts)]
#[instruction(company_name: String)]

pub struct CreatingVestingAccount<'info> {
  #[account(mut)]

  pub signer: Signer<'info>,

  #[account(
    init,
    space = 8 + VestingAccount::INIT_SPACE,
    payer = signer,
    seeds = [company_name.as_ref()],
    bump,
  )]

  pub vesting_account: Account<'info, VestingAccount>,

  pub mint: InterfaceAccount<'info, Mint>,

}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount {
  pub owner: PubKey,
  pub mint: PubKey,
  pub treasury_token_account: PubKey,
  #[max_len(50)]
  pub company_name: String,
  pub treasury_bump: u8,
  pub bump: u8,
}



