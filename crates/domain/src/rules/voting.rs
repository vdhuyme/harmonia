use crate::Result;

/// Vote on a queue item
/// Prevents duplicate votes from same user
pub fn validate_vote(user_has_voted: bool) -> Result<()> {
    if user_has_voted {
        return Err(crate::AppError::DuplicateVote);
    }
    Ok(())
}

/// Undo a vote
pub fn validate_unvote(user_has_voted: bool) -> Result<()> {
    if !user_has_voted {
        return Err(crate::AppError::ValidationFailed(
            "User has not voted for this item".to_string(),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_vote_allows_first_vote() {
        let result = validate_vote(false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_vote_prevents_duplicate() {
        let result = validate_vote(true);
        assert!(result.is_err());
        match result {
            Err(crate::AppError::DuplicateVote) => (),
            _ => panic!("Expected DuplicateVote error"),
        }
    }

    #[test]
    fn test_validate_unvote_allows_existing_vote() {
        let result = validate_unvote(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_unvote_prevents_non_existent() {
        let result = validate_unvote(false);
        assert!(result.is_err());
    }
}
