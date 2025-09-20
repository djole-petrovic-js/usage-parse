//! Struct for collecting all required arguments from the command line.
//!
//! When adding new arguments, a new field should be added, as well as a corresponding extractor code (potentially with validation).
use super::super::FormatterFactory;
#[derive(Debug)]
pub struct CLIArgs {
    logs_dir: String,
    formatter: String,
}

impl CLIArgs {
    /// *Simple utility function, for extracting the log dir*
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// let cli_args = CLIArgs::build(&mut vec!["--log_dir=test_dir".to_string()].into_iter()).unwrap();
    ///
    /// assert_eq!(cli_args.get_logs_dir(), "test_dir");
    /// ```
    pub fn get_logs_dir(&self) -> &String {
        &self.logs_dir
    }

    /// *Get the chosen formatter*
    ///
    /// ---
    ///
    /// # Example
    ///
    /// ```
    /// let cli_args = CLIArgs::build(&mut vec![
    ///     "--log_dir=test_dir".to_string(),
    ///     "--formatter=json".to_string(),
    /// ].into_iter()).unwrap();
    ///
    /// assert_eq!(cli_args.get_formatter(), "json");
    /// ```
    pub fn get_formatter(&self) -> &String {
        &self.formatter
    }
    /// *Get required arguments from the command line*
    ///
    /// ---
    ///
    /// All required arguments must be present, and be in valid format. Leave optional as they are.
    /// Method could return `Err(String)`, if something went wrong, so make sure to check for that.
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// - `env_iterator` - Any iterator type, that can provide CLI arguments one by one.
    ///
    /// ## Example
    ///
    /// ```
    /// let cli_args = CLIArgs::build(&mut vec!["--log_dir=test_dir".to_string()].into_iter());
    ///
    /// assert!(!cli_args.is_err());
    /// ```
    pub fn build(env_iterator: &mut dyn Iterator<Item = String>) -> Result<CLIArgs, String> {
        let mut logs_dir = String::new();
        let mut formatter = String::from("stdout");

        for arg in env_iterator {
            let mut split = arg.split("=");

            let arg_name = match split.next() {
                Some(arg_name) => arg_name,
                None => {
                    return Err(
                        "Could not extract the first part from the argument. Check your input!"
                            .to_string(),
                    );
                }
            };

            let arg_value = match split.next() {
                Some(arg_value) => arg_value,
                None => {
                    return Err(
                        "Could not extract the second part from the argument. Check your input!"
                            .to_string(),
                    );
                }
            };

            match arg_name {
                // Required
                "--log_dir" | "-ld" => {
                    logs_dir = arg_value.trim().to_owned();
                }
                // Optional
                // If present, must be a known formatter
                "--formatter" | "-fmt" => {
                    if FormatterFactory::resolve_formatter(arg_value).is_err() {
                        return Err("Unknown formatter. Run the CLI with the -h, to get the list of the available formatters".to_string());
                    }

                    formatter.clear();
                    formatter.push_str(arg_value);
                }

                unknown_arg_name => {
                    return Err(format!("Unknown parameter: {}", unknown_arg_name));
                }
            }
        }
        // Check if any of the variables are left empty.
        if logs_dir.is_empty() {
            return Err("Logs directory parameter is missing! Check your input".to_string());
        }

        let cli_args: CLIArgs = CLIArgs {
            logs_dir,
            formatter,
        };

        Ok(cli_args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_required_args_builds_with_full_paths() {
        let args_list = vec!["--log_dir=test_dir".to_string()];

        let cli_args = CLIArgs::build(&mut args_list.into_iter()).unwrap();

        assert_eq!(cli_args.get_logs_dir(), "test_dir");
    }

    #[test]
    fn test_correct_required_args_builds_with_shortned_paths() {
        let args_list = vec!["-ld=test_dir".to_string()];

        let cli_args = CLIArgs::build(&mut args_list.into_iter()).unwrap();

        assert_eq!(cli_args.get_logs_dir(), "test_dir");
    }

    #[test]
    fn test_incorrect_args_builds() {
        let cli_args = CLIArgs::build(&mut vec![].into_iter());

        assert!(cli_args.is_err());
        assert!(cli_args.unwrap_err() == "Logs directory parameter is missing! Check your input");

        let cli_args = CLIArgs::build(&mut vec!["--log_dir".to_string()].into_iter());

        assert!(cli_args.is_err());
        assert!(
            cli_args.unwrap_err()
                == "Could not extract the second part from the argument. Check your input!"
        );

        let cli_args =
            CLIArgs::build(&mut vec!["--non_existant_arg=some_value".to_string()].into_iter());

        assert!(cli_args.is_err());
        assert!(cli_args.unwrap_err() == "Unknown parameter: --non_existant_arg");
    }

    #[test]
    fn test_formatter_arg() {
        // Just to test if didn't returned an error, since this is an optional param.
        let cli_args = CLIArgs::build(&mut vec!["--log_dir=test_dir".to_string()].into_iter());

        assert!(!cli_args.is_err());

        let cli_args = CLIArgs::build(
            &mut vec![
                "--log_dir=test_dir".to_string(),
                "--formatter=json".to_string(),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(cli_args.get_formatter(), "json");

        let cli_args = CLIArgs::build(
            &mut vec!["-ld=test_dir".to_string(), "-fmt=json".to_string()].into_iter(),
        )
        .unwrap();

        assert_eq!(cli_args.get_formatter(), "json");

        let cli_args = CLIArgs::build(
            &mut vec![
                "--log_dir=test_dir".to_string(),
                "--formatter=stdout".to_string(),
            ]
            .into_iter(),
        )
        .unwrap();

        assert_eq!(cli_args.get_formatter(), "stdout");

        let cli_args = CLIArgs::build(
            &mut vec!["-ld=test_dir".to_string(), "-fmt=stdout".to_string()].into_iter(),
        )
        .unwrap();

        assert_eq!(cli_args.get_formatter(), "stdout");

        let cli_args = CLIArgs::build(
            &mut vec![
                "-ld=test_dir".to_string(),
                "-fmt=unknown_formatter".to_string(),
            ]
            .into_iter(),
        );

        assert!(cli_args.unwrap_err().contains("Unknown formatter"));
    }
}
