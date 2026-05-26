use super::*;

#[test]
fn new_language_code_with_valid_fr_creates_instance() {
    let code = LanguageCode::new("FR");
    assert_eq!(code, LanguageCode::FR);
}

#[test]
fn new_language_code_with_valid_en_creates_instance() {
    let code = LanguageCode::new("EN");
    assert_eq!(code, LanguageCode::EN);
}

#[test]
fn try_new_language_code_with_invalid_code_returns_error() {
    let result = LanguageCode::try_new("DE");
    assert!(result.is_err());
}

#[test]
fn try_new_language_code_with_invalid_code_contains_msg() {
    let result = LanguageCode::try_new("DE");
    match result {
        Err(CardParsingError::InvalidLanguageCode(msg)) => assert_eq!(msg, "DE"),
        _ => panic!("Expected InvalidLanguageCode variant"),
    }
}

#[test]
fn new_language_code_is_case_insensitive() {
    let code_lower = LanguageCode::new("fr");
    let code_mixed = LanguageCode::new("Fr");
    assert_eq!(code_lower, LanguageCode::FR);
    assert_eq!(code_mixed, LanguageCode::FR);
}

#[test]
fn display_language_code_returns_correct_string() {
    assert_eq!(LanguageCode::FR.to_string(), "FR");
    assert_eq!(LanguageCode::EN.to_string(), "EN");
}
