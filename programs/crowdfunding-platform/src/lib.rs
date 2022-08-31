use anchor_lang::prelude::*;
use std::mem;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("ARsfrJ4ZYVKHsnvNM3GU4HtRsV9Nru9XiXgDAEMYJaxm");

#[program]
pub mod crowdfunding_platform {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, description: String, fulfilled: u64) -> Result<()> {

        let campaign = &mut ctx.accounts.campaign;

        campaign.campaign_owner = *ctx.accounts.user.key;
        campaign.campaign_amounts = 0;
        campaign.campaign_descriptions = description;
        campaign.campaign_fulfilled = fulfilled;

        
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;
        let user = &ctx.accounts.user;
        if campaign.campaign_owner != *user.key {
            return Err(ProgramError::IncorrectProgramId);
        }
        let rent_balance = Rent::get()?.minimum_balance(campaign.to_account_info().data_len());
        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount {
            return Err(ProgramError::InsufficientFunds);
        }
        **campaign.to_account_info().try_borrow_mut_lamports()? -= amount;
        **user.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }

    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        let inst = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(), 
            &ctx.accounts.campaign.key(),
            amount
        );

        let res = anchor_lang::solana_program::program::invoke(
            &inst,
            
            &[
                ctx.accounts.campaign.to_account_info(),
                ctx.accounts.user.to_account_info()
            ],
        );

        let acc = &mut ctx.accounts.campaign;

        acc.campaign_fulfilled += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = user,
        space = 9000,
        seeds = [b"Campaign".as_ref(), user.key().as_ref()],
        bump
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>, 
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>, 
}


#[account]
pub struct Campaign{
    pub campaign_owner: Pubkey,
    pub campaign_amounts: u64,
    pub campaign_descriptions: String,
    pub campaign_fulfilled: u64, 
}
