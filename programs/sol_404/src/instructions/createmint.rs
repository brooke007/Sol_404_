// 创建了jellymint, usdcmint, nftmint, 以及jelly ATA, usdc ATA

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::{accounts::Metadata as MetadataAccount, types::DataV2},
        CreateMetadataAccountsV3, Metadata,
    },
    token::{Mint, Token, TokenAccount},
};
use solana_program::{pubkey, pubkey::Pubkey};


const ADMIN_PUBKEY: Pubkey = pubkey!("D5KC5YnyPAPnCKpAbFaUUyQCo6mz6mKnVrgot9Tme1Mg");


pub fn createmint(
    ctx: Context<CreateMint>,
    jelly_uri: String,
    usdc_uri: String,
    nft_uri: String,
    jelly_name: String,
    usdc_name: String,
    nft_name: String,
    jelly_symbol: String,
    usdc_symbol: String,
    nft_symbol: String,
) -> Result<()> {

    //initialize jelly mint
    let seeds = b"jellya";
    let bump = ctx.bumps.jelly_token_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    // On-chain token metadata for the mint
    let data_v2 = DataV2 {
        name: jelly_name,
        symbol: jelly_symbol,
        uri: jelly_uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // CPI Context
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.jelly_metadata_account.to_account_info(), // the metadata account being created
            mint: ctx.accounts.jelly_token_mint.to_account_info(), // the mint account of the metadata account
            mint_authority: ctx.accounts.jelly_token_mint.to_account_info(), // the mint authority of the mint account
            update_authority: ctx.accounts.jelly_token_mint.to_account_info(), // the update authority of the metadata account
            payer: ctx.accounts.admin.to_account_info(), // the payer for creating the metadata account
            system_program: ctx.accounts.system_program.to_account_info(), // the system program account, required when creating new accounts
            rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
        },
        signer, // pda signer
    );

    create_metadata_accounts_v3(
        cpi_ctx, // cpi context
        data_v2, // token metadata
        true,    // is_mutable
        true,    // update_authority_is_signer
        None,    // collection details
    )?;


    //initialize usdc mint
    let seeds = b"usdc";
    let bump = ctx.bumps.usdc_token_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    // CPI Context
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.usdc_metadata_account.to_account_info(), // the metadata account being created
            mint: ctx.accounts.usdc_token_mint.to_account_info(), // the mint account of the metadata account
            mint_authority: ctx.accounts.usdc_token_mint.to_account_info(), // the mint authority of the mint account
            update_authority: ctx.accounts.usdc_token_mint.to_account_info(), // the update authority of the metadata account
            payer: ctx.accounts.admin.to_account_info(), // the payer for creating the metadata account
            system_program: ctx.accounts.system_program.to_account_info(), // the system program account, required when creating new accounts
            rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
        },
        signer, // pda signer
    );

    let data_v2 = DataV2 {
        name: usdc_name,
        symbol: usdc_symbol,
        uri: usdc_uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    create_metadata_accounts_v3(
        cpi_ctx, // cpi context
        data_v2, // token metadata
        true,    // is_mutable
        true,    // update_authority_is_signer
        None,    // collection details
    )?;

  

    let seeds = b"nft";
    let bump = ctx.bumps.nft_mint;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    let data_v2 = DataV2 {
        name: nft_name,
        symbol: nft_symbol,
        uri: nft_uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

     // CPI Context
     let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_metadata_program.to_account_info(),
        CreateMetadataAccountsV3 {
            metadata: ctx.accounts.nft_metadata_account.to_account_info(), // the metadata account being created
            mint: ctx.accounts.nft_mint.to_account_info(), // the mint account of the metadata account
            mint_authority: ctx.accounts.nft_mint.to_account_info(), // the mint authority of the mint account
            update_authority: ctx.accounts.nft_mint.to_account_info(), // the update authority of the metadata account
            payer: ctx.accounts.admin.to_account_info(), // the payer for creating the metadata account
            system_program: ctx.accounts.system_program.to_account_info(), // the system program account, required when creating new accounts
            rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
        },
        signer, // pda signer
    );

    create_metadata_accounts_v3(
        cpi_ctx, // cpi context
        data_v2, // token metadata
        true,    // is_mutable
        true,    // update_authority_is_signer
        None,    // collection details
    )?;


    Ok(())
}


#[derive(Accounts)]
pub struct CreateMint<'info> {
    // Use ADMIN_PUBKEY as constraint, only the specified admin can invoke this instruction
    #[account(
        mut,
        address = ADMIN_PUBKEY
    )]
    pub admin: Signer<'info>,

    // The PDA is both the address of the mint account and the mint authority
    #[account(
        init,
        seeds = [b"jellya"],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = jelly_token_mint,
    )]
    pub jelly_token_mint: Account<'info, Mint>,
    
    ///CHECK: Using "address" constraint to validate metadata account address, this account is created via CPI in the instruction
    #[account(
        mut,
        address = MetadataAccount::find_pda(&jelly_token_mint.key()).0,
    )]
    pub jelly_metadata_account: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [b"usdc"],
        bump,
        payer = admin,
        mint::decimals = 9,
        mint::authority = usdc_token_mint,
    )]
    pub usdc_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        address = MetadataAccount::find_pda(&usdc_token_mint.key()).0,
    )]
    /// CHECK: The `usdc_metadata_account` is an UncheckedAccount, which means that no runtime safety checks
    pub usdc_metadata_account: UncheckedAccount<'info>,

    // nft mint
    #[account(
        init,
        seeds = [b"nft"],
        bump,
        payer = admin,
        mint::decimals = 0,
        mint::authority = nft_mint,
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        address = MetadataAccount::find_pda(&nft_mint.key()).0,
    )]
    /// CHECK: The `usdc_metadata_account` is an UncheckedAccount, which means that no runtime safety checks
    pub nft_metadata_account: UncheckedAccount<'info>,

    

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

