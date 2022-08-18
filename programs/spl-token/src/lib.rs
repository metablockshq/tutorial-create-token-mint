use anchor_lang::prelude::*;
use anchor_spl::{token::{self, Mint, Token, TokenAccount}, associated_token::AssociatedToken};

declare_id!("29iiLtNregFkwH4n4K95GrKYcGUGC3F6D5thPE2jWQQs");

#[program]
pub mod spl_token {
    use super::*;

    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.payer.key();
        vault.spl_token_mint_bump = *ctx.bumps.get("spl_token_mint").unwrap();
        vault.bump = *ctx.bumps.get("vault").unwrap();
        vault.spl_token_mint = ctx.accounts.spl_token_mint.key();

        Ok(())
    }

    pub fn transfer_mint(ctx : Context<TransferMint>) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: ctx.accounts.spl_token_mint.to_account_info(),
                to: ctx.accounts.payer_mint_ata.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );
        token::mint_to(cpi_context, 1)?;
        Ok(())
    }
}


// create mint context
#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
        init,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump,
        payer = payer,
        mint::authority = payer,
        mint::decimals = 0,
        mint::freeze_authority = payer
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 2

    pub system_program: Program<'info, System>, // ---> 3
    pub token_program: Program<'info, Token>,   // ---> 4
    // this is required for spl token mint
    pub rent: Sysvar<'info, Rent>, // ---> 5

    #[account(
        init, 
        space = 8 + Vault::LEN,
        seeds = [
            b"vault"
        ],
        bump,
        payer = payer 
    )]
    pub vault : Account<'info, Vault>, // ---> 6
}

// Transfer mint context
#[derive(Accounts)]
pub struct TransferMint<'info> {
    #[account(
        mut,
         seeds = [
            b"spl-token-mint".as_ref(),
         ],
        bump = vault.spl_token_mint_bump,
    )]
    pub spl_token_mint: Account<'info, Mint>, // ---> 1

    #[account(
        seeds = [
            b"vault"
        ],
        bump = vault.bump, // --> 2
    )]
    pub vault : Account<'info, Vault>, 

    #[account(
        init,
        payer = payer,
        associated_token::mint = spl_token_mint,
        associated_token::authority = payer
    )]
    pub payer_mint_ata: Box<Account<'info, TokenAccount>>,  // --> 3

    #[account(mut)]
    pub payer: Signer<'info>, // ---> 4

    pub system_program: Program<'info, System>, // ---> 5
    pub token_program: Program<'info, Token>,   // ---> 6
    
    pub rent: Sysvar<'info, Rent>, // ---> 7

    pub associated_token_program : Program<'info, AssociatedToken>  // ---> 8

}



// Store the state 
#[account]
pub struct Vault { //  ---> 7
    bump : u8, //1
    spl_token_mint_bump:u8, // 1
    authority : Pubkey, //32
    spl_token_mint : Pubkey //32
}

impl Vault {
    pub const LEN: usize =1 + 1 + 32 + 32;
}