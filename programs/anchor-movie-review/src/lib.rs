use anchor_lang::prelude::*;

declare_id!("76exkBtD2k2KBqB7Q61rbw73bcGVSn9tY8RW5U4JhQi2");

const DISCRIMINATOR: usize = 8;

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}

fn check_params(title: &String, description: &String, rating: u8) -> Result<()> {
    require!(
        rating >= MIN_RATING && rating <= MAX_RATING,
        MovieReviewError::InvalidRating
    );

    require!(
        title.len() <= MAX_TITLE_LENGTH,
        MovieReviewError::TitleTooLong
    );

    require!(
        description.len() <= MAX_DESCRIPTION_LENGTH,
        MovieReviewError::DescriptionTooLong
    );

    msg!("Movie Review Account Created");
    msg!("Title: {}", title);
    msg!("Description: {}", description);
    msg!("Rating: {}", rating);
    Ok(())
}

#[program]
pub mod anchor_movie_review {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        if let Err(err) = check_params(&title, &description, rating) {
            return Err(err);
        }

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        if let Err(err) = check_params(&title, &description, rating) {
            return Err(err);
        }

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    /// 创建评论的用户
    pub reviewer: Pubkey, // 32
    /// 电影的评级
    pub rating: u8, // 1
    /// 电影的标题
    #[max_len(20)]
    pub title: String, // 4 + len()
    /// 评论的内容
    #[max_len(50)]
    pub description: String, // 4 + len()
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + MovieAccountState::INIT_SPACE
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
