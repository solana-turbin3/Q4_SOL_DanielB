use anchor_lang::prelude::*;
use crate::contexts::admin::InitializeAdmin;

pub fn initialize_admin(ctx: Context<InitializeAdmin>, admin_pda: Pubkey) -> Result<()> {
    let admin_account = &mut ctx.accounts.admin_pda;

    // Set the derived `admin_pda` as the key for the account.
    admin_account.key = admin_pda;

    Ok(())
}
