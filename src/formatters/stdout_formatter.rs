//! Standard formatting for the aggregate usage.
//!
//! Mostly used for just outputting / debugging, or inserting into a log.
use super::super::log_parser_lib::owner_usage_struct::OwnerUsage;
use super::formatter_trait::Formatter;
use std::collections::HashMap;

pub struct StdoutFormatter;

impl Formatter for StdoutFormatter {
    /// Standard formating, as plain strings.
    ///
    /// ---
    ///
    /// For the Arguments and Example, see [`Formatter`] trait.
    fn format(&self, aggregate: &HashMap<u32, OwnerUsage>) -> String {
        let mut output = String::new();

        output
            .push_str(String::from(format!("---------------------------------------\n")).as_str());

        for (owner_id, owner_usage) in aggregate {
            output.push_str(String::from(format!("Owner with id: {}\n\n", owner_id)).as_str());
            output.push_str(String::from(format!("Usage\n\n")).as_str());
            output.push_str(
                String::from(format!(
                    "  Video plays: {}\n",
                    owner_usage.get_video_plays()
                ))
                .as_str(),
            );
            output.push_str(
                String::from(format!(
                    "  Ad Impressions: {}\n",
                    owner_usage.get_ad_impressions()
                ))
                .as_str(),
            );

            output.push_str(
                String::from(format!("---------------------------------------\n")).as_str(),
            );
        }

        output
    }
    /// @see [`Formatter`] trait.
    fn identifier(&self) -> &'static str {
        return "stdout";
    }
}
