pub mod error;
pub mod instructions;
pub mod state;


use anchor_lang::prelude::*;
use instructions::*;
declare_id!("9kfP7Pvonn3sV7vLWVyEFbtH9LxuXN9B7UENQAZ4T39w");

#[program]
pub mod sol_404 {
    use super::*;

    pub fn createmint(ctx: Context<CreateMint>,jelly_uri: String, usdc_uri: String, nft_uri: String, jelly_name: String, usdc_name: String, nft_name: String, jelly_symbol: String, usdc_symbol: String, nft_symbol: String) -> Result<()>{
        instructions::createmint(ctx, jelly_uri, usdc_uri, nft_uri, jelly_name, usdc_name, nft_name, jelly_symbol, usdc_symbol, nft_symbol)
    }

    pub fn transfer(ctx: Context<Trans>) -> Result<()> {
        instructions::transfera(ctx)
    }

    // pay usdt to buy our tokens
    pub fn buytokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        instructions::buytokens(ctx, amount)
    }

    // pay amount token to get usdt return
    pub fn usetokens(ctx: Context<UseTokens>, amount: u64) -> Result<()> {
        instructions::usetokens(ctx, amount)
    }
}

