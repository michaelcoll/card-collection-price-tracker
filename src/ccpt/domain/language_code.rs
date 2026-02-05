use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LanguageCode {
    FR,
    EN,
}

impl LanguageCode {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let s_ref = s.as_ref();
        match s_ref.to_uppercase().as_str() {
            "FR" => LanguageCode::FR,
            "EN" => LanguageCode::EN,
            _ => panic!("invalid language code : {}", s_ref),
        }
    }
}

impl Display for LanguageCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageCode::FR => write!(f, "FR"),
            LanguageCode::EN => write!(f, "EN"),
        }
    }
}

#[cfg(test)]
mod tests {
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
    #[should_panic(expected = "invalid language code : DE")]
    fn new_language_code_with_invalid_code_panics() {
        LanguageCode::new("DE");
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
}
