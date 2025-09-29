//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    // Get the reaction and tweet accounts
    let reaction = &ctx.accounts.tweet_reaction;
    let tweet = &mut ctx.accounts.tweet;

    // Verify that the reaction belongs to the reaction_author
    if reaction.reaction_author != ctx.accounts.reaction_author.key() {
        return Err(ProgramError::InvalidAccountData.into());
    }

    // Decrement the appropriate counter on the tweet
    match reaction.reaction {
        ReactionType::Like => {
            if tweet.likes == 0 {
                return Err(TwitterError::MinLikesReached.into());
            }
            tweet.likes = tweet.likes.checked_sub(1).ok_or(TwitterError::MinLikesReached)?;
        }
        ReactionType::Dislike => {
            if tweet.dislikes == 0 {
                return Err(TwitterError::MinDislikesReached.into());
            }
            tweet.dislikes = tweet.dislikes.checked_sub(1).ok_or(TwitterError::MinDislikesReached)?;
        }
    }

    // Close the reaction account and return rent to reaction_author
    let reaction_account_info = ctx.accounts.tweet_reaction.to_account_info();
    let reaction_author_account_info = ctx.accounts.reaction_author.to_account_info();
    let lamports = reaction_account_info.lamports();
    **reaction_account_info.lamports.borrow_mut() = 0;
    **reaction_author_account_info.lamports.borrow_mut() += lamports;

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    #[account(
        mut,
        close = reaction_author,
        seeds = [TWEET_REACTION_SEED.as_bytes(), reaction_author.key().as_ref(), tweet.key().as_ref()],
        bump = tweet_reaction.bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub reaction_author: Signer<'info>,   
}
