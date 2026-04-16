/// Custom UUID validator
pub fn validate_uuid(uuid: &str) -> Result<(), validator::ValidationError> {
    uuid::Uuid::parse_str(uuid)
        .map_err(|_| validator::ValidationError::new("invalid_uuid"))?;
    Ok(())
}

/// Custom provider validator
pub fn validate_provider(
    provider: &str,
) -> Result<(), validator::ValidationError> {
    let lower = provider.to_lowercase();
    if lower == "spotify" || lower == "youtube" {
        Ok(())
    } else {
        Err(validator::ValidationError::new("invalid_provider"))
    }
}

/// Custom string length validator for non-empty strings
pub fn validate_non_empty_string(
    value: &str,
) -> Result<(), validator::ValidationError> {
    if value.trim().is_empty() {
        Err(validator::ValidationError::new("empty_or_whitespace"))
    } else {
        Ok(())
    }
}
