//! A struct for parsing all URL lines in a given file, extracting usage parameters from the query strings.
//! 
//! Every line has this format : https://www.mysite.com/pixel.gif?o=123&v=2222&i=555
//! 
//! All parameters must be valid integers.
//! Owner must exist in the URL.
//! Other parameters are optional, and multiple of them can appear at the same time, or just one of them.
//! In the future, consider returning the ids of the entities associated with this events (like player id, ad unit id, video id etc.)
//! 
//! For now, this struct only records how many times an event happend ( like video plays for example .)
use std::collections::HashMap;
use std::io::BufRead;

use super::log_parser_error::LogParserError;
use super::owner_usage_struct::OwnerUsage;
use super::query_string_params_enum::QueryStringParameters;
use super::utils::{get_query_string, get_query_string_parameter_value};

pub struct LogParser<'a> {
    file_name: &'a str,
}

impl<'a> LogParser<'a> {
    /// *Construct a new struct, with a given file_name*
    ///
    /// ---
    ///
    /// ## Arguments
    /// - `file_name` - Full path to a given log file
    ///
    /// ## Example
    ///
    /// ```
    /// let log_file_path = "log_file.txt";
    /// let log_parser_instance = LogParser::new(&log_file_path);
    /// ```
    pub fn new(file_name: &'a str) -> Self {
        Self { file_name }
    }
    /// *For a given usage param, increase the usage by 1.*
    ///
    /// ---
    /// Every param is found in every log line at most once. So it makes sense to just increment the fields.
    ///
    /// Note: The method could return None, indicating that it could add to an existing usage param (Overflow happened for example).
    /// So make sure to check for the None variant.
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// - `owner_usage` - A struct holding the current usage for a given owner
    /// - `query_string` - Query string extracted from a log line
    /// - `parameter` - Usage parameter as a single char (v | i | p | o etc etc)
    /// - `map_field` - Which field on the usage struct to increment.
    ///
    /// # Example
    ///
    /// ```
    /// let test_log_file = "log.txt";
    /// let mut owner_usage_hash_map = OwnerUsage::default();
    /// let query_string = "o=111&v=222&i=333";
    /// let log_parser = LogParser::new(test_log_file);
    ///
    /// log_parser.increment_hash_map_field(
    ///     &mut owner_usage_hash_map,
    ///     &query_string,
    ///     &QueryStringParameters::resolve_query_string_parameter(&QueryStringParameters::VideoId),
    ///     "video_plays",
    /// ).unwrap();
    /// ```
    fn increment_hash_map_field(
        &self,
        owner_usage: &mut OwnerUsage,
        query_string: &str,
        parameter: &char,
        map_field: &str,
    ) -> Option<()> {
        // It's ok if the parameter is missing. Not every single log line has to contain every parameter.
        if let Some(value) = get_query_string_parameter_value(&query_string, &parameter) {
            // At this point, the method could error.
            // Parse error should definitely be signaled.
            if let Ok(_) = value.parse::<u32>() {
                // map_field must be a valid one, and the addition must succeed.
                let add_result = match map_field {
                    "video_plays" => owner_usage.add_video_plays(1),
                    "ad_impressions" => owner_usage.add_ad_impressions(1),
                    _ => None,
                };

                return match add_result {
                    Some(_) => Some(()),
                    None => None,
                };
            } else {
                return None;
            }
        }

        Some(())
    }
    /// *Start the parsing process for a given log file*
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// Method takes no arguments. All parameters are provided in the constructor
    ///
    /// ## Example
    ///
    /// ```
    /// let test_log_file = "log.txt";
    /// let log_parser = LogParser::new(test_log_file);
    ///
    /// let parse_result = log_parser.parse();
    ///
    /// assert!(parse_result.is_ok());
    ///
    /// let owner_usage_hash_map: std::collections::HashMap<u32, OwnerUsage> = parse_result.unwrap();
    /// ```
    pub fn parse(&self) -> Result<HashMap<u32, OwnerUsage>, LogParserError> {
        let file = match std::fs::File::open(self.file_name) {
            Ok(file) => file,
            Err(error) => {
                return Err(LogParserError::Io(error));
            }
        };

        let mut reader = std::io::BufReader::new(file);
        let mut line_string = String::new();
        let mut output: HashMap<u32, OwnerUsage> = HashMap::new();

        loop {
            let read_result = reader.read_line(&mut line_string);

            match read_result {
                Ok(line_size) => {
                    if line_size == 0 {
                        break;
                    }
                    // Query string must exists (everyting after the >>> ? <<< character in the string)
                    let query_string = match get_query_string(&line_string) {
                        Some(query_string) => query_string,
                        None => {
                            return Err(LogParserError::Custom(
                                "No query string found!".to_string(),
                            ));
                        }
                    };
                    // Owner is required, and must be parsed properly.
                    let owner_id = match get_query_string_parameter_value(&query_string, &'o') {
                        Some(owner_id) => match owner_id.parse::<u32>() {
                            Ok(owner_id) => owner_id,
                            Err(error) => return Err(LogParserError::ParseIntError(error)),
                        },
                        None => {
                            return Err(LogParserError::Custom(
                                "No owner id found in the query string!".to_string(),
                            ));
                        }
                    };

                    let owner_usage_instance =
                        output.entry(owner_id).or_insert(OwnerUsage::default());

                    if self
                        .increment_hash_map_field(
                            owner_usage_instance,
                            &query_string,
                            &QueryStringParameters::resolve_query_string_parameter(
                                &QueryStringParameters::VideoId,
                            ),
                            "video_plays",
                        )
                        .is_none()
                    {
                        return Err(LogParserError::Custom("Failed to add to the video_plays! Possible overflow situation, or a parse error".to_string()));
                    }

                    if self
                        .increment_hash_map_field(
                            owner_usage_instance,
                            &query_string,
                            &QueryStringParameters::resolve_query_string_parameter(
                                &QueryStringParameters::AdUnitId,
                            ),
                            "ad_impressions",
                        )
                        .is_none()
                    {
                        return Err(LogParserError::Custom("Failed to add to the ad_impressions! Possible overflow situation, or a parse error".to_string()));
                    }

                    line_string.clear();
                }

                Err(error) => {
                    return Err(LogParserError::Io(error));
                }
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_log_parser() {
        let test_log_path = "test_log.txt";
        let mut file_handle = std::fs::File::create(test_log_path).unwrap();
        let log_lines = r#"https://www.mysite.com/pixel.gif?o=123&v=2222&i=123
https://www.mysite.com/pixel.gif?o=123&v=3333
https://www.mysite.com/pixel.gif?o=123&v=4444
https://www.mysite.com/pixel.gif?o=444&v=1111&i=4444
"#;

        file_handle.write_all(log_lines.as_bytes()).unwrap();

        let log_parser = LogParser::new(&test_log_path);
        let owner_usage_hash_map = log_parser.parse();

        std::fs::remove_file(test_log_path).unwrap();

        assert!(!owner_usage_hash_map.is_err());

        let owner_usage_hash_map = owner_usage_hash_map.unwrap();

        // Check first Owner
        let owner_usage_for_owner_123 = owner_usage_hash_map.get(&123).unwrap();

        assert_eq!(owner_usage_for_owner_123.get_video_plays(), 3);
        assert_eq!(owner_usage_for_owner_123.get_ad_impressions(), 1);
        // Check second owner
        let owner_usage_hash_map_for_owner_444 = owner_usage_hash_map.get(&444).unwrap();

        assert_eq!(owner_usage_hash_map_for_owner_444.get_video_plays(), 1);
        assert_eq!(owner_usage_hash_map_for_owner_444.get_ad_impressions(), 1);
    }

    #[test]
    fn test_increment_hash_map_field() {
        let test_log_file = "not_exist_log.txt";
        let log_parser = LogParser::new(test_log_file);
        let mut owner_usage_hash_map = OwnerUsage::default();
        let query_string = "o=111&v=222&i=333";

        log_parser
            .increment_hash_map_field(
                &mut owner_usage_hash_map,
                &query_string,
                &QueryStringParameters::resolve_query_string_parameter(
                    &QueryStringParameters::VideoId,
                ),
                "video_plays",
            )
            .unwrap();

        log_parser
            .increment_hash_map_field(
                &mut owner_usage_hash_map,
                &query_string,
                &QueryStringParameters::resolve_query_string_parameter(
                    &QueryStringParameters::AdUnitId,
                ),
                "ad_impressions",
            )
            .unwrap();

        assert_eq!(owner_usage_hash_map.get_video_plays(), 1);
        assert_eq!(owner_usage_hash_map.get_ad_impressions(), 1);

        let query_string_with_v_only = "o=111&v=333";

        log_parser
            .increment_hash_map_field(
                &mut owner_usage_hash_map,
                &query_string_with_v_only,
                &QueryStringParameters::resolve_query_string_parameter(
                    &QueryStringParameters::VideoId,
                ),
                "video_plays",
            )
            .unwrap();

        assert_eq!(owner_usage_hash_map.get_video_plays(), 2);
        assert_eq!(owner_usage_hash_map.get_ad_impressions(), 1);

        let query_string_with_i_only = "o=111&i=333";

        log_parser
            .increment_hash_map_field(
                &mut owner_usage_hash_map,
                &query_string_with_i_only,
                &QueryStringParameters::resolve_query_string_parameter(
                    &QueryStringParameters::AdUnitId,
                ),
                "ad_impressions",
            )
            .unwrap();

        assert_eq!(owner_usage_hash_map.get_video_plays(), 2);
        assert_eq!(owner_usage_hash_map.get_ad_impressions(), 2);
        // Test the overflow
        let mut owner_usage_hash_map = OwnerUsage::new(u32::MAX, u32::MAX);

        let increment_result_none = log_parser.increment_hash_map_field(
            &mut owner_usage_hash_map,
            &query_string,
            &QueryStringParameters::resolve_query_string_parameter(&QueryStringParameters::VideoId),
            "video_plays",
        );

        assert_eq!(increment_result_none, None);

        let increment_result_none = log_parser.increment_hash_map_field(
            &mut owner_usage_hash_map,
            &query_string,
            &QueryStringParameters::resolve_query_string_parameter(
                &QueryStringParameters::AdUnitId,
            ),
            "ad_impressions",
        );

        assert_eq!(increment_result_none, None);

        let test_log_file = "log.txt";
        let mut owner_usage_hash_map = OwnerUsage::default();
        let query_string = "o=111&v=222&i=333";
        let log_parser = LogParser::new(test_log_file);

        log_parser
            .increment_hash_map_field(
                &mut owner_usage_hash_map,
                &query_string,
                &QueryStringParameters::resolve_query_string_parameter(
                    &QueryStringParameters::VideoId,
                ),
                "video_plays",
            )
            .unwrap();
    }
}
