use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("CMTq3EDLYh5DMHWdG9kZoC3fgtcXCARsD7gLip783KXw");

#[program]
pub mod pda_wallet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        let account_data = &mut ctx.accounts.pda_account;
        msg!("user key :{:?}", ctx.accounts.user.key);
        msg!("pda_account :{:?}", ctx.bumps.pda_account);
        msg!("name :{:?}", name);

        account_data.user = *ctx.accounts.user.key;
        account_data.bump = ctx.bumps.pda_account;

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn transfer(ctx: Context<Transfer>, amount: f64) -> Result<()> {
        msg!("Try send sols: {}", amount);
        **ctx.accounts.pda_account.try_borrow_mut_lamports()? -=amount as u64;
        **ctx.accounts.recipient.try_borrow_mut_lamports()? +=amount as u64;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    user: Signer<'info>,

    #[account(
        init,
        seeds = [b"user", name.as_bytes(), user.key().as_ref()],
        bump,
        payer = user,
        space = 8 + size_of::<DataAccount>()
    )]
    pub pda_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    user: Signer<'info>,

    /// CHECK:
    #[account(
        mut,
        owner = id()
    )]
    pub pda_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub recipient: SystemAccount<'info>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default, InitSpace)]
pub struct DataAccount {
    pub user: Pubkey,
    pub bump: u8,
}
