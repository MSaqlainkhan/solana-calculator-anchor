use anchor_lang::prelude::*;

declare_id!("LEX2HNNLUG5v6XY7BHyga25HjpfPteQrqB5Df421N7B");

#[program]
pub mod solana_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.cal_acc.result = 0;
        ctx.accounts.cal_acc.payer = ctx.accounts.payer.key();
        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: u8, b: u8) -> Result<()> {
        ctx.accounts.cal_acc.result = a + b;
        msg!(
            "Addition of {} and {} is done. Result = {}",
            a,
            b,
            ctx.accounts.cal_acc.result
        );
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, a: u8, b: u8) -> Result<()> {
        ctx.accounts.cal_acc.result = a - b;
        msg!(
            "Subtraction done. Result = {}",
            ctx.accounts.cal_acc.result
        );
        Ok(())
    }
}

#[account]
pub struct CalResult {
    pub result: u8,
    pub payer: Pubkey,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + 1 + 32)]
    pub cal_acc: Account<'info, CalResult>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub cal_acc: Account<'info, CalResult>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub cal_acc: Account<'info, CalResult>,
}
