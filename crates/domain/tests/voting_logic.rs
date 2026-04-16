use domain::rules::voting::*;

#[test]
fn test_validate_vote_first_time() {
    // User hasn't voted yet
    let result = validate_vote(false);
    assert!(result.is_ok());
}

#[test]
fn test_validate_vote_duplicate() {
    // User already voted
    let result = validate_vote(true);
    assert!(result.is_err());
    match result {
        Err(core::AppError::DuplicateVote) => (),
        _ => panic!("Expected DuplicateVote error"),
    }
}

#[test]
fn test_validate_unvote_success() {
    // User has voted and wants to unvote
    let result = validate_unvote(true);
    assert!(result.is_ok());
}

#[test]
fn test_validate_unvote_no_vote() {
    // User hasn't voted and tries to unvote
    let result = validate_unvote(false);
    assert!(result.is_err());
    match result {
        Err(core::AppError::ValidationFailed(msg)) => {
            assert!(msg.contains("has not voted"))
        }
        _ => panic!("Expected ValidationFailed error"),
    }
}
