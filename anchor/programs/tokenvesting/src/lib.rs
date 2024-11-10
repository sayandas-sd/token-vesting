#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod tokenvesting {
    use super::*;

    pub fn creating_vesting_account(ctx: Contex<CreatingVestingAccount>, company_name: String) -> Result<()> {
      *ctx.accounts.vesting_account = VestingAccount {
        owner: ctx.accounts.signer.key(), 
        mint: ctx.accounts.mint.key(),
        treasury_token_account: ctx.accounts.treasury_token_account.key(), 
        company_name,
        treasury_bump: ctx.bumps.treasury_token_account, 
        bump: ctx.bumps.vesting_account,
      };
      
      Ok(())
    }

    pub fn creating_employee_account(
      ctx: Contex<CreatingEmployeeAccount>, 
      start_time: i64, 
      end_time: i64, 
      clifftime: u64,
      total_amount: u64
    ) -> Result<()> {
      *ctx.accounts.employee_account = EmployeeAccount {
        beneficiary: ctx.acccounts.benificiary.key(),
        start_time,
        end_time,
        cliff_time,
        total_amount,
        total_withdrawn: 0,
        vesting_account: ctx.acccounts.vesting_account.key(),
        bump: ctx.bumps.employee_account,
      }
      Ok(())
    }


    pub fn claim_tokens(ctx: Contex<ClaimTokens>, company_name: String) -> Result<()> {
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

  #[account(
    init,
    token::mint = mint,
    token::authority = treasury_token_account,
    payer = signer,
    seeds = ["vesting_treasury", company_name.as_bytes()]
  )]

  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

  pub system_program: Program<'info, System>,

  pub token_program: Interface<'info, TokenInterface>,
}



#[deirve(Accounts)]
pub struct CreatingEmployeeAccount<'info> {
  #[account(mut)]
  pub owner:  Signer<'info>,
  pub beneficiary: SystemAccount<'info>,
  #[account(
    has_one = owner
  )]
  pub vesting_account: Account<'info, VestingAccount>,

  #[account(
    init,
    space = 8 + EmployeeAccount::INIT_SPACE,
    payer = owner,
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
    bump,
  )]

  pub employee_account: Account<'info, EmployeeAccount>, 

  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
  #[account(mut)]

  pub beneficiary: Signer<'info>,

  #[acccount(
    mut,
    seeds = [b"employee_vesting", beneficiary.key().as_ref(), vesting_account.key().as_ref()],
    bump = employee_account.bump,
    has_one = beneficiary,
    has_one = vesting_account,

  )]

  pub employee_account: Accounts<'info, EmployeeAccount>,

  //check correct beneficiary or employee can claim the token

  #[acccount(
    mut,
    seeds = [company_name.as_bytes()],
    bump = vesting_account.bump,
    has_one = treasury_token_account,
    has_one = mint,
  )]

  pub vesting_account:  Account<'info, VestingAccount>,

  pub mint: InterfaceAccount<'info, Mint>,

  #[account(mut)]
  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

  #[acccount(mut)]
  init_if_needed,
  payer = beneficiary,
  associated_token::Mint = mint,
  associated_token::authority = beneficiary,
  associated_token::token_program = token_program,

  pub employee_token_account: InterfaceAccount<'info, TokenAccount>,
  pub token_program: Interface<'info, TokenInterface>,
  pub associated_token_program: Program<'info, AssociatedToken>,
  pub system_program: Program<'info, System>,

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


#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
  pub beneficiary: PubKey,
  pub start_time: i64,
  pub end_time: i64,
  pub cliff_time: i64,
  pub vesting_account: PubKey,
  pub total_amount: u64,
  pub total_withdrawn: u64,
  pub bump: u8
}

