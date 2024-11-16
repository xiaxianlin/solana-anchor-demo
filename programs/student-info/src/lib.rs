use anchor_lang::prelude::*;

declare_id!("EuD6qQyMPmdjTem7HsKtHAuQcmfNJZ8vsWC8S6fBAc9m");

const DISCRIMINATOR: usize = 8;

const MAX_NAME_LENGTH: usize = 20;
const MAX_INTRO_LENGTH: usize = 50;

#[error_code]
enum StudentError {
    #[msg("Movie Title too long")]
    NameTooLong,
    #[msg("Movie Description too long")]
    IntroTooLong,
}

fn check_params(name: &String, intro: &String) -> Result<()> {
    require!(name.len() <= MAX_NAME_LENGTH, StudentError::NameTooLong);
    require!(intro.len() <= MAX_INTRO_LENGTH, StudentError::IntroTooLong);

    msg!("Student Info");
    msg!("Name: {}", name);
    msg!("Intro: {}", intro);
    Ok(())
}

#[program]
pub mod student_info {
    use super::*;

    pub fn add_student(ctx: Context<AddStudent>, name: String, intro: String) -> Result<()> {
        if let Err(err) = check_params(&name, &intro) {
            return Err(err);
        }

        let student = &mut ctx.accounts.student;
        student.creator = ctx.accounts.initializer.key();
        student.name = name;
        student.intro = intro;

        Ok(())
    }

    pub fn update_student(ctx: Context<UpdateStudent>, name: String, intro: String) -> Result<()> {
        if let Err(err) = check_params(&name, &intro) {
            return Err(err);
        }

        let student = &mut ctx.accounts.student;
        student.intro = intro;

        Ok(())
    }

    pub fn delete_student(_ctx: Context<DeleteStudent>, name: String) -> Result<()> {
        msg!("Student named {} is deleted", name);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Student {
    /// 学生姓名
    #[max_len(20)]
    pub name: String,
    /// 学生介绍
    #[max_len(50)]
    pub intro: String,
    /// 创建人
    pub creator: Pubkey,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct AddStudent<'info> {
    /// 添加的学生
    #[account(
        init,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + Student::INIT_SPACE
    )]
    pub student: Account<'info, Student>,
    /// 创建人，也是交易付款人
    #[account(mut)]
    pub initializer: Signer<'info>,
    /// 初始化程序的系统程序
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct UpdateStudent<'info> {
    #[account(
        mut,
        seeds = [name.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + Student::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
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
