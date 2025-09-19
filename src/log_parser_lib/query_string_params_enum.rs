//! Enum to represent usage parameter, that can be found in a query string.
//!
//! Every parameter in the query string is represented by a single char, to save space in the url.
//! Every value is an integer for now.
//! List of every parameter is listed bellow.
//!
//! Make sure to when adding new ones, to update the tests as well.
pub enum QueryStringParameters {
    /// Video ID, of the video that was played / started.
    VideoId,
    /// Ad Unit ID. Fired when an Ad started.
    AdUnitId,
}

impl QueryStringParameters {
    /// *Return a Char representation for a given usage parameter.*
    ///
    /// ---
    ///
    /// All usage parameters are found a single characters (see example bellow.).
    /// This is just a helper, so we don't have to use magic single characters everywhere.
    ///
    /// ## Arguments
    ///
    /// - `x` - Field on the QueryStringParameters enum.
    ///
    /// ## Example
    ///
    /// ```
    /// let field = QueryStringParameters::VideoId;
    ///
    /// assert_eq!(QueryStringParameters::resolve_query_string_parameter(&field), 'v')
    /// ```
    pub fn resolve_query_string_parameter(x: &QueryStringParameters) -> char {
        match x {
            QueryStringParameters::VideoId => 'v',
            QueryStringParameters::AdUnitId => 'i',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_resolve_random_parameters_successfully() {
        let video_id_param = QueryStringParameters::VideoId;

        assert_eq!(
            QueryStringParameters::resolve_query_string_parameter(&video_id_param),
            'v'
        );

        let ad_unit_param = QueryStringParameters::AdUnitId;

        assert_eq!(
            QueryStringParameters::resolve_query_string_parameter(&ad_unit_param),
            'i'
        );
    }
}
