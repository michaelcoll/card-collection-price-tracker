#[derive(Debug, Clone)]
pub enum AppError {
    ParseError {
        line: usize,
        field: &'static str,
        value: String,
    },
    WrongFormat(String),
    CalculationFailed(String),
    RepositoryError(String),
    PriceNotFound,
    CallError(String),
}

impl From<AppError> for String {
    fn from(val: AppError) -> String {
        match val {
            AppError::ParseError { line, field, value } => format!(
                "Line {}: invalid {} '{}' (must be a valid value)",
                line, field, value
            ),
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
    fn app_error_to_string_for_parse_error() {
        let error = AppError::ParseError {
            line: 1,
            field: "quantity",
            value: "abc".to_string(),
        };
        let error_message: String = error.into();
        assert_eq!(
            error_message,
            "Line 1: invalid quantity 'abc' (must be a valid value)"
        );
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
