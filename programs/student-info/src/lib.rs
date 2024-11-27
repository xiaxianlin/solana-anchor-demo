use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("GtjERV4BsnvPzamnWjDE3gVsbqPYTLYkUZw44GeSYKpD");

const PUBKEY_SIZE: usize = 32;
const ANCHOR_DISCRIMINATOR: usize = 8;
const U8_SIZE: usize = 1;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 20;
const MAX_INTRO_LENGTH: usize = 50;

#[program]
pub mod student_info {
    use anchor_spl::token::{mint_to, MintTo};

    use super::*;

    pub fn add_student(ctx: Context<AddStudent>, name: String, intro: String) -> Result<()> {
        require!(name.len() <= MAX_NAME_LENGTH, StudentError::NameTooLong);
        require!(intro.len() <= MAX_INTRO_LENGTH, StudentError::IntroTooLong);
        msg!("Student created");
        msg!("Name: {}", name);
        msg!("Intro: {}", intro);

        let student = &mut ctx.accounts.student;
        student.creator = ctx.accounts.initializer.key();
        student.name = name;
        student.intro = intro;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.mint]]],
            ),
            10 * 10 ^ 6,
        )?;

        Ok(())
    }

    pub fn update_student(ctx: Context<UpdateStudent>, name: String, intro: String) -> Result<()> {
        require!(name.len() <= MAX_NAME_LENGTH, StudentError::NameTooLong);
        require!(intro.len() <= MAX_INTRO_LENGTH, StudentError::IntroTooLong);
        msg!("Student updated");
        msg!("Name: {}", name);
        msg!("Intro: {}", intro);

        let student = &mut ctx.accounts.student;
        student.intro = intro;

        Ok(())
    }

    pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        msg!("Student named {} is deleted", name);
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        msg!("Token mint initialized");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, intro:String)]
pub struct AddStudent<'info> {
    /// 添加的学生
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = Student::INIT_SPACE + name.len() + intro.len()
    )]
    pub student: Account<'info, Student>,
    /// 创建人，也是交易付款人
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// 初始化程序的系统程序
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct DeleteStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub student: Account<'info, Student>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Student {
    pub creator: Pubkey,
    pub name: String,
    pub intro: String,
}

impl Space for Student {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + U8_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}

#[error_code]
enum StudentError {
    #[msg("Movie Title too long")]
    NameTooLong,
    #[msg("Movie Description too long")]
    IntroTooLong,
}
