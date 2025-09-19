//! A factory struct, to return a correct Formatter instance, depending on the CLI arguments.
//!
//! Note: Think about refactoring the Box<dyn Formatter> argument, to avoid heap usage and allocation.
use super::formatter_trait::Formatter;
use super::json_formatter::JsonFormatter;
use super::stdout_formatter::StdoutFormatter;

pub struct FormatterFactory {}

impl FormatterFactory {
    /// *Method for resolving and returing a correct instance of Formatter.*
    ///
    /// ---
    ///
    /// All available formatters should be here.
    ///
    /// Note: It could return an error, indicating that the user has misspelled a formatter name or something.
    pub fn resolve_formatter(formatter_from_cli: &str) -> Result<Box<dyn Formatter>, String> {
        match formatter_from_cli {
            "json" => Ok(Box::new(JsonFormatter {})),

            "stdout" => Ok(Box::new(StdoutFormatter {})),

            unknown_formatter => Err(String::from(format!(
                "Unknown formatter: {}",
                unknown_formatter
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_resolve_correct_formatter() {
        assert_eq!(
            FormatterFactory::resolve_formatter(&"json")
                .unwrap()
                .identifier(),
            "json"
        );
        assert_eq!(
            FormatterFactory::resolve_formatter(&"stdout")
                .unwrap()
                .identifier(),
            "stdout"
        );
        assert!(FormatterFactory::resolve_formatter(&"unknown").is_err());
    }
}
