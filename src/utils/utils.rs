/// *Try to get all files, inside of a given directory*
///
/// ---
///
/// Method will include additional checks, such as verifying if a file name is properly encoded. Therefore, not all files are guaranteed to be included in the returned vector.
/// Ordering of the files is also not guaranteed. So the example below does not include the .get(0) and .get(1) checks. Use sorting if ordering is needed.
///
/// ---
///
/// ## Arguments
///
/// - `dir` - Directory to read
///
/// ## Example
///
/// ```
/// // Assuming there is a logs directory, with log1.txt and log2.txt inside.
/// let files = get_file_names("logs");
///
/// assert!(!files.is_empty());
/// assert!(files.len() == 2);
/// ```
pub fn get_file_names(dir: &str) -> Vec<String> {
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                /*
                 * into_string, on the file_name, could return an error, indicating that the file name is not in a valid utf8 format.
                 * So just skip this for now.
                 */
                let log_file = match entry.file_name().into_string() {
                    Ok(file_name) => file_name,
                    Err(_) => {
                        continue;
                    }
                };

                files.push(log_file);
            }
        }
    }

    return files;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creating_and_reading_dir() {
        let test_log_dir = "test_log_dir";
        let test_log_file_one = "log1.txt";
        let test_log_file_one_full_path = format!("{}/{}", test_log_dir, test_log_file_one);
        let test_log_file_two = "log2.txt";
        let test_log_file_two_full_path = format!("{}/{}", test_log_dir, test_log_file_two);

        std::fs::create_dir(test_log_dir).unwrap();

        std::fs::File::create(&test_log_file_one_full_path).unwrap();
        std::fs::File::create(&test_log_file_two_full_path).unwrap();
        // Create the instance, and immediately remove the directory and the files.
        // If something fails below, we'll have leftover resources.
        let mut test_log_files = get_file_names(&test_log_dir);
        
        std::fs::remove_file(&test_log_file_one_full_path).unwrap();
        std::fs::remove_file(&test_log_file_two_full_path).unwrap();
        std::fs::remove_dir(test_log_dir).unwrap();

        test_log_files.sort();

        assert!(!test_log_files.is_empty());
        assert!(test_log_files.len() == 2);
        assert_eq!(test_log_files.get(0), Some(&test_log_file_one.to_string()));
        assert_eq!(test_log_files.get(1), Some(&test_log_file_two.to_string()));
    }
}
