use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug)]
pub enum AppError {
    ParseError(),
    WrongFormat(String),
    CalculationFailed(String),
    RepositoryError(String),
    PriceNotFound,
    CallError(String),
}

impl From<ParseIntError> for AppError {
    fn from(_err: ParseIntError) -> Self {
        AppError::ParseError()
    }
}

impl From<ParseFloatError> for AppError {
    fn from(_err: ParseFloatError) -> Self {
        AppError::ParseError()
    }
}

impl From<AppError> for String {
    fn from(val: AppError) -> String {
        match val {
            AppError::ParseError() => "Parse Error".to_string(),
            AppError::WrongFormat(msg) => msg,
            AppError::CalculationFailed(msg) => msg,
            AppError::RepositoryError(msg) => msg,
            AppError::PriceNotFound => "Price not found".to_string(),
            AppError::CallError(msg) => msg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_error_converts_to_parse_error() {
        let parse_error: AppError = "123abc".parse::<i32>().unwrap_err().into();
        assert!(matches!(parse_error, AppError::ParseError()));
    }

    #[test]
    fn parse_float_error_converts_to_parse_error() {
        let parse_error: AppError = "123.abc".parse::<f64>().unwrap_err().into();
        assert!(matches!(parse_error, AppError::ParseError()));
    }

    #[test]
    fn app_error_to_string_for_parse_error() {
        let error = AppError::ParseError();
        let error_message: String = error.into();
        assert_eq!(error_message, "Parse Error");
    }

    #[test]
    fn app_error_to_string_for_wrong_format() {
        let error = AppError::WrongFormat("Invalid format".to_string());
        let error_message: String = error.into();
        assert_eq!(error_message, "Invalid format");
    }

    #[test]
    fn app_error_to_string_for_calculation_failed() {
        let error = AppError::CalculationFailed("Calculation failed".to_string());
        let error_message: String = error.into();
        assert_eq!(error_message, "Calculation failed");
    }

    #[test]
    fn app_error_to_string_for_repository_error() {
        let error = AppError::RepositoryError("Repository error".to_string());
        let error_message: String = error.into();
        assert_eq!(error_message, "Repository error");
    }

    #[test]
    fn app_error_to_string_for_price_not_found() {
        let error = AppError::PriceNotFound;
        let error_message: String = error.into();
        assert_eq!(error_message, "Price not found");
    }

    #[test]
    fn app_error_to_string_for_call_error() {
        let error = AppError::CallError("Call failed".to_string());
        let error_message: String = error.into();
        assert_eq!(error_message, "Call failed");
    }
}
