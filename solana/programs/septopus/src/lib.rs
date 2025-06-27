#![allow(unexpected_cfgs)]  //solve the #[program] warning issue

use anchor_lang::prelude::*;

declare_id!("7tUr1JZECqmPAHqew3sjrzmygXsxCfzWoqfXaLsn6AZF");

use {
    init::*,
};
pub mod init;

#[program]
pub mod septopus {
    use super::*;

    ///init whole system
    pub fn init(
        ctx: Context<InitDeclaration>,
    ) -> Result<()> {
        init::entry(ctx)
    }

}
