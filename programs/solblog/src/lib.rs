use anchor_lang::prelude::*;

declare_id!("4ZV2ShGBbnWuY6me2QN8FWGFxeEWGR52WwiWnuvqsiE5");

#[program]
pub mod solblog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 566)]
    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BlogAccount {
    pub authority: Pubkey,
    pub latest_post: Vec<u8>,
}
