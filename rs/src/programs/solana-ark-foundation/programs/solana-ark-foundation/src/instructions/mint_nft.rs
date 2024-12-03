use anchor_lang::prelude::*;
use anchor_spl::token::{self, MintTo};
use mpl_token_metadata::instructions::CreateV1Builder;
use mpl_token_metadata::types::{TokenStandard, PrintSupply};
use crate::contexts::mint_nft::MintVeterinaryCabinetNFT;
use crate::errors::ErrorCode;

pub fn mint_veterinary_cabinet_nft(
    ctx: Context<MintVeterinaryCabinetNFT>,
    metadata_uri: String, // URI for metadata (e.g., IPFS URL)
    name: String,
    symbol: String,
) -> Result<()> {
    // Mint the NFT

       // Ensure the caller is a recognized veterinary cabinet
    if ctx.accounts.cabinet.id != ctx.accounts.payer.key() {
        return err!(ErrorCode::UnauthorizedAccess);
    }
    
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.admin_pda.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::mint_to(cpi_ctx, 1)?; // Mint 1 token

    // Prepare metadata creation instruction using the updated CreateV1Builder
    let instruction = CreateV1Builder::new()
        .metadata(ctx.accounts.metadata_account.key())       // Metadata account public key
        .master_edition(Some(ctx.accounts.master_edition.key())) // Master edition account public key
        .mint(ctx.accounts.mint.key(), true)                 // Mint account public key and verify
        .authority(ctx.accounts.admin_pda.key())             // Authority public key
        .payer(ctx.accounts.payer.key())                     // Separate payer account
        .update_authority(ctx.accounts.admin_pda.key(), true) // Update authority public key and verify
        .is_mutable(true)                                    // Metadata is mutable
        .primary_sale_happened(false)                        // Primary sale not happened
        .name(name)                                          // NFT name
        .symbol(symbol)                                      // NFT symbol
        .uri(metadata_uri)                                   // Metadata URI
        .seller_fee_basis_points(500)                        // 5% royalties
        .token_standard(TokenStandard::NonFungible)          // NFT type
        .print_supply(PrintSupply::Zero)                     // No prints
        .instruction();                                      // Generate the instruction

    // CPI Invocation
    anchor_lang::solana_program::program::invoke_signed(
        &instruction,
        &[ // Include payer in the list of required accounts
            ctx.accounts.metadata_account.to_account_info(),
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.admin_pda.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.payer.to_account_info(),
        ],
        &[&[ // Signer seeds remain for the admin PDA
            b"admin_pda",
            &[ctx.bumps.admin_pda], // Access bump directly
        ]],
    )?;

    Ok(())
}
