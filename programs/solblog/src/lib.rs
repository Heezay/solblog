use anchor_lang::prelude::*;
// https://learn.figment.io/tutorials/build-a-blog-dapp-using-anchor
declare_id!("4ZV2ShGBbnWuY6me2QN8FWGFxeEWGR52WwiWnuvqsiE5");

#[program]
pub mod solblog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let b_p_a = &mut ctx.accounts.blog_account;
        b_p_a.authority = *ctx.accounts.authority.key;
        Ok(())
    }

    pub fn make_post(ctx: Context<MakePost>, new_post: Vec<u8>) -> Result<()> {
        let post = std::str::from_utf8(&new_post) // convert the array of bytes into a string slice
            .map_err(|err| {
                msg!("Invalid UTF-8, from byte {}", err.valid_up_to());
                ProgramError::InvalidInstructionData
            })?;
        msg!(post);

        let b_acc = &mut ctx.accounts.blog_account;
        b_acc.latest_post = new_post;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 566)]
    pub blog_account: Account<'info, BlogAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct BlogAccount {
    pub authority: Pubkey,
    pub latest_post: Vec<u8>,
}

#[derive(Accounts)]
pub struct MakePost<'info> {
    #[account(mut, has_one = authority)]
    pub blog_account: Account<'info, BlogAccount>,
    pub authority: Signer<'info>,
}
