use anchor_lang::prelude::*;

///Accounts space setting
pub const ANCHOR_DESCRIMINATOR_SIZE: usize = 8;
pub const SOLANA_PDA_LEN:usize=8;
pub const LS_NAME_MAP_SIZE:usize=1000;          //name map size
pub const LS_WHITELIST_MAP_SIZE:usize=1000;     //whitelist map size

pub const LS_ROOT_ACCOUNT:&str="GTNgXEzmG2E2d9yX8fwueP4bD2WCgJ3mqvt7sQj6CYYr";             //root of LuckySig program

///PDA accounts seeds
pub const LS_SEEDS_CATEGORY:&[u8;7]=b"dec_cat";


#[account]
#[derive(InitSpace)]
pub struct CategoryCounter {
    pub value: u64,
}

impl CategoryCounter {
    pub fn inc(&mut self, amount:u64) {
        self.value += amount
    }

    ///!important, only on Devnet
    //FIXME, DEBUG only, need to remove when deploy on mainnet
    pub fn set(&mut self, amount:u64) {
        self.value = amount
    }

    ///!important, only on Devnet
    //FIXME, DEBUG only, need to remove when deploy on mainnet
    pub fn max(&mut self) {
        self.value = LS_MAX_AMOUNT_OF_SINGLE_GENE
    }
}