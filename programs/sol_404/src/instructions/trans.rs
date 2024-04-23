// 给buyer先转点usdc 不然玩不了
use std::default;

use anchor_lang::{accounts::program, prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, TokenAccount, mint_to, transfer, Mint, MintTo, Transfer, Token}
};
use solana_program::{pubkey, pubkey::Pubkey};

use crate::error::*;

pub fn transfera(
    ctx: Context<Trans>
) -> Result<()>{
    let seeds = b"usdc";
    let bump = ctx.bumps.usdc_token_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.usdc_token_mint.to_account_info(), // mint account of token to mint
            to: ctx.accounts.buyer_usdc_account.to_account_info(), // worker token account to mint to
            authority: ctx.accounts.usdc_token_mint.to_account_info(), // pda is used as both address of mint and mint authority
        },
        signer,
    );

    mint_to(cpi_ctx, 1000000000000000)?;

    Ok(())
}


#[derive(Accounts)]
pub struct Trans<'info>{
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_usdc_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"usdc"],
        bump,
    )]
    pub usdc_token_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}