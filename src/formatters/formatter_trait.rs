//! Trait to represent all available formatters.
//!
//! Should be used with a factory struct. A factory should decide which formatter should be returned.
//!
//! When adding new formatters, they should implement the trait as well.
use super::super::log_parser_lib::owner_usage_struct::OwnerUsage;
use std::collections::HashMap;
pub trait Formatter {
    /// *Format the aggregate as a String*
    ///
    /// ---
    ///
    /// It is returned as a string because the caller should decide what to do with the output.
    /// The caller can just print it to the stdout, or send a HTTP request, or something similar.
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// - `aggregate` - Aggregate usage data for all users
    ///
    /// ## Example
    ///
    /// ```
    /// let formatter = FormatterFactory::resolve_formatter(&"json");
    /// let formatter = formatter.unwrap();
    ///
    /// let result = formatter.format(&aggregate);
    /// ```
    fn format(&self, aggregate: &HashMap<u32, OwnerUsage>) -> String;
    /// *Return the identifier of the formatter, as a plain string*
    ///
    /// ---
    ///
    /// Currently used just for testing, to make sure the correct formatter is returned from the factory.
    /// Apparently, it is not possible to determine the instance, without adding some ugly code.
    ///
    /// ---
    ///
    /// ## Example
    ///
    /// ```
    /// assert_eq!(FormatterFactory::resolve_formatter(&"json").unwrap().identifier(), "json");
    /// ```
    #[allow(dead_code)]
    fn identifier(&self) -> &'static str;
}
