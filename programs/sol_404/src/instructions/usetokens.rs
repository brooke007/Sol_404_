// 给我们发token 我们给他usdc 然后销毁他的nft
use std::default;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::{accounts::Metadata as MetadataAccount, types::DataV2},
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, transfer, Mint, MintTo, Transfer, Token, TokenAccount, burn, Burn},
};
use solana_program::{pubkey, pubkey::Pubkey};

const ADMIN_PUBKEY: Pubkey = pubkey!("D5KC5YnyPAPnCKpAbFaUUyQCo6mz6mKnVrgot9Tme1Mg");


pub fn usetokens(
    ctx: Context<UseTokens>,
    amount: u64
) -> Result<()>{
    //代码中得检查用户有没有usdc账户 因为他之前可能充token充完了, 没有usdc账号 返回error

    let seeds = b"usdc";
    let bump = ctx.bumps.usdc_token_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    // 我们给buyer转usdc
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.usdc_token_mint.to_account_info(), // mint account of token to mint
            to: ctx.accounts.buyer_usdc_account.to_account_info(), // worker token account to mint to
            authority: ctx.accounts.usdc_token_mint.to_account_info(), // pda is used as both address of mint and mint authority
        },
        signer, // pda signer
    );
    //把token转换为usdc(这里这样的转换出来的数应该是有一定的问题的)
    let transfer_usdc_amount = amount
        .checked_mul(10u64.pow(ctx.accounts.usdc_token_mint.decimals as u32))
        .unwrap();
    
    mint_to(cpi_ctx, transfer_usdc_amount)?;



    // buyer给我们转token
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.buyer_token_account.to_account_info(), 
            to: ctx.accounts.jelly_token_account.to_account_info(), 
            authority: ctx.accounts.buyer.to_account_info(), 
        },
    );

    let transfer_usdc_amount = amount
        .checked_mul(10u64.pow(ctx.accounts.usdc_token_mint.decimals as u32))
        .unwrap();
    
    transfer(cpi_ctx, transfer_usdc_amount)?;

    // buyer的nft烧掉 还得算一下 得burn掉多少个nft
    //这里的reward_token_min为nft的铸币厂的地址
 

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.nft_mint.to_account_info(),
            from: ctx.accounts.buyer.to_account_info(),
            authority: ctx.accounts.buyer.to_account_info(),
        },
    );

    // Burn 1 token, accounting for decimals of mint
    let remainder = amount % 10;
    let burn_amount = 
    if remainder == 0 {
        remainder
    } else {
        remainder + 1
    };

    burn(cpi_ctx, burn_amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct UseTokens<'info>{
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        address = ADMIN_PUBKEY
    )]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::mint = jelly_token_mint,
        associated_token::authority = admin
    )]
    pub jelly_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = jelly_token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = usdc_token_mint,
        associated_token::authority = buyer
    )]
    pub buyer_usdc_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = nft_mint,
        associated_token::authority = buyer
    )]
    pub buyer_nft_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"usdc"],
        bump,
    )]
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"jellya"],
        bump,
    )]
    pub jelly_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"nft"],
        bump,
    )]
    pub nft_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}