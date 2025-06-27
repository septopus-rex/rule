use {
    anchor_lang::prelude::*,
    anchor_spl::{
        metadata::{
            create_metadata_accounts_v3, mpl_token_metadata::types::DataV2,
            CreateMetadataAccountsV3, Metadata,
        },
        token::{Mint, Token},
    },
};
use std::str::FromStr;

use crate::constants::{
    SOLANA_PDA_LEN,
};

/********************************************************************/
/************************ Public Functions **************************/
/********************************************************************/

pub fn entry(
    ctx: Context<InitDeclaration>,        //default from system
) -> Result<()> {

    Ok(())
}
/********************************************************************/
/*********************** Private Functions **************************/
/********************************************************************/

fn is_root_account(check_pubkey:Pubkey,root:&str) -> bool{
    let pubkey = solana_program::pubkey::Pubkey::from_str(root).expect("Invalid pubkey");
    let pubkey_bytes: [u8; 32] = pubkey.to_bytes();
    let manage_pubkey = anchor_lang::prelude::Pubkey::new_from_array(pubkey_bytes);
    if check_pubkey != manage_pubkey {
        return false;
    }
    return true;
}

/********************************************************************/
/************************* Data Structure ***************************/
/********************************************************************/

#[derive(Accounts)]
#[instruction(recipient:String)]
pub struct InitDeclaration<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
}