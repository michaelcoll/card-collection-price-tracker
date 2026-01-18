use crate::domain::error::CardParsingError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCode(String);

impl SetCode {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, CardParsingError> {
        let s_ref = s.as_ref();
        if s_ref.chars().count() == 3 {
            Ok(SetCode(s_ref.to_string()))
        } else {
            Err(CardParsingError::InvalidSetCode(format!(
                "set code must be exactly 3 characters (got {})",
                s_ref
            )))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SetName {
    pub code: SetCode,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::CardParsingError;

    #[test]
    fn new_creates_set_code_with_valid_input() {
        let result = SetCode::new("ABC");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), SetCode("ABC".to_string()));
    }

    #[test]
    fn new_returns_error_for_short_code() {
        let result = SetCode::new("AB");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CardParsingError::InvalidSetCode(
                "set code must be exactly 3 characters (got AB)".to_string()
            )
        );
    }

    #[test]
    fn new_returns_error_for_long_code() {
        let result = SetCode::new("ABCD");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CardParsingError::InvalidSetCode(
                "set code must be exactly 3 characters (got ABCD)".to_string()
            )
        );
    }

    #[test]
    fn new_handles_empty_string() {
        let result = SetCode::new("");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            CardParsingError::InvalidSetCode(
                "set code must be exactly 3 characters (got )".to_string()
            )
        );
    }

    #[test]
    fn set_name_creation_with_valid_code_and_name() {
        let code = SetCode::new("XYZ").unwrap();
        let name = "Test Set".to_string();
        let set_name = SetName {
            code,
            name: name.clone(),
        };
        assert_eq!(set_name.code, SetCode("XYZ".to_string()));
        assert_eq!(set_name.name, name);
    }
}
