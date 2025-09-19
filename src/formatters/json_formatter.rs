//! Format the aggregate usage, as a JSON string.
//!
//! Could be used to be sent to a remote server or something similar.
use super::super::log_parser_lib::owner_usage_struct::OwnerUsage;
use super::formatter_trait::Formatter;
use std::collections::HashMap;

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    /// Format the aggregate as a JSON String.
    ///
    /// ---
    ///
    /// For the Arguments and Example, see [`Formatter`] trait.
    fn format(&self, aggregate: &HashMap<u32, OwnerUsage>) -> String {
        let mut json_output = String::new();

        json_output.push_str("[");

        let mut iterator = aggregate.into_iter().peekable();

        loop {
            match iterator.next() {
                Some((owner_id, owner_usage_hash_map)) => {
                    let raw = format!(
                        r#"{{
                        "owner_id": {owner_id},
                        "usage": {{
                            "video_plays": {},
                            "ad_impressions": {}
                        }}
                    }}"#,
                        owner_usage_hash_map.get_video_plays(),
                        owner_usage_hash_map.get_ad_impressions(),
                    );

                    json_output.push_str(&raw);

                    if iterator.peek().is_some() {
                        json_output.push_str(&",");
                    }
                }

                None => {
                    break;
                }
            }
        }

        json_output.push_str("]");

        json_output
    }

    /// @see [`Formatter`] trait.
    fn identifier(&self) -> &'static str {
        return "json";
    }
}
