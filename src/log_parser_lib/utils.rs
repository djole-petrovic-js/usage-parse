//! Utility functions for the log parser lib.

/// *Find the query string in an url.*
///
/// ---
///
/// In a full URL, query string is considered group of characters, after the '?' characted.
///
/// ---
///
/// Note: This method could return None, so make sure to handle that.
///
/// ## Arguments
///
/// - `line` - A line in a log file, which is the full URL.
///
/// ## Example
///
/// ```
/// let line = "https://mysite.com/route?param=value".to_string();
///
/// assert_eq!(get_query_string(&line), Some("param=value"));
/// ```
pub fn get_query_string(line: &String) -> Option<&str> {
    let bytes = line.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'?' {
            let query_string = &line[i + 1..];
            // For a url like this, https://www.mysite.com?, query string will be empty.
            // So signal that it is not correct query string.
            return if !query_string.is_empty() {
                Some(query_string)
            } else {
                None
            };
        }
    }

    None
}
/// *Get a value from a specific query string parameter*
///
/// ## Arguments
///
/// - `query_string` - Entire query string
/// - `parameter` - Parameter name as a single char.
///
/// # Example
///
/// ```
/// let query_string = "o=111&v=222&i=333";
///
/// assert_eq!(get_query_string_parameter_value(&query_string, &'o'), Some("111"));
/// ```
///
pub fn get_query_string_parameter_value<'a>(
    query_string: &'a str,
    parameter: &char,
) -> Option<&'a str> {
    let bytes = query_string.as_bytes();
    let needle = (*parameter) as u8;

    for (index, &item) in bytes.iter().enumerate() {
        if item == needle && bytes[index + 1] == b'=' {
            let needle_index = index + 2;
            // Found the parameter, for example o= etc etc
            // now get the end of it, with the & sign, for example o=123&
            for next_item in needle_index..bytes.len() {
                if bytes[next_item] == b'&' {
                    return Some(&query_string[needle_index..next_item]);
                }
            }
            // Needle is now the last parameter in the query string.
            // Check if the last character is the new line character. Make sure to skip it. Otherwise, parsing as integer wont work.
            if bytes[bytes.len() - 1] == b'\n' {
                return Some(&query_string[needle_index..bytes.len() - 2]);
            }
            // This is probably the last line, and it doesn't contain the new line char.
            return Some(&query_string[needle_index..]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_string() {
        assert_eq!(
            get_query_string(&"https://www.mysite.com".to_string()),
            None
        );
        // Test when there is nothing after the first ? char.
        assert_eq!(
            get_query_string(&"https://www.mysite.com?".to_string()),
            None
        );

        assert_eq!(
            get_query_string(&"https://mysite.com/route?param=value".to_string()),
            Some("param=value")
        );
        // For now, query string is everything after the first ? char.
        assert_eq!(
            get_query_string(
                &"https://mysite.com/route?param=value&param2=param2?param3=param3".to_string()
            ),
            Some("param=value&param2=param2?param3=param3")
        );

        assert_eq!(
            get_query_string(
                &"https://mysite.com/route?param=value&param2=param2?param3=param3".to_string()
            ),
            Some("param=value&param2=param2?param3=param3")
        );
    }

    #[test]
    fn test_get_query_string_parameter_value() {
        let query_string = "o=111&v=222&i=333";

        assert_eq!(
            get_query_string_parameter_value(&query_string, &'o'),
            Some("111")
        );
        assert_eq!(
            get_query_string_parameter_value(&query_string, &'v'),
            Some("222")
        );
        assert_eq!(
            get_query_string_parameter_value(&query_string, &'i'),
            Some("333")
        );
        assert_eq!(get_query_string_parameter_value(&query_string, &'X'), None);
        assert_eq!(get_query_string_parameter_value(&"", &'W'), None);
    }
}
