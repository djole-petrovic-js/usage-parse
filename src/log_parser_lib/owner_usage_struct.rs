/// Struct representing an usage group for a single owner.
///
/// All new usage metrics should be added here...
///
/// Note: Adding a value to the struct usage field can potentially lead to an overflow. In product, it will wrap up to the begining, which is bad.
/// So always handle that, it must not overflow.
///
/// Note: u32 type is used for all fields for convinience.
/// In production, change it to the mysql fields data types for example.
#[derive(Default, Debug)]
pub struct OwnerUsage {
    video_plays: u32,
    ad_impressions: u32,
}

impl OwnerUsage {
    #[cfg(test)]
    /// *Returns a owner usage struct, with predefined starting usages*
    ///
    /// ---
    ///
    /// Note: This is not only used for tests, to test the overflow when adding to the fields.
    /// Since other methods are just incrementing by 1, it would take forever to increment a field, up until the u32::MAX, to test the overflow.
    /// Refactor this if necessery.
    pub fn new(video_plays: u32, ad_impressions: u32) -> Self {
        Self {
            video_plays,
            ad_impressions,
        }
    }

    /// *Return the video plays param*
    pub fn get_video_plays(&self) -> u32 {
        self.video_plays
    }
    /// *Return the ad impressions param*
    pub fn get_ad_impressions(&self) -> u32 {
        self.ad_impressions
    }
    /// *Try to add an integer to the video_plays param*
    ///
    /// ---
    ///
    /// Note: Method is using the `checked_add()` method, to check for overflow.
    /// Returning None should signal an error.
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// - `param` - Integer to append
    ///
    /// ## Example
    ///
    /// ```
    /// let owner_usage = OwnerUsage::default()
    /// let add_result = owner_usage.add_video_plays(10);
    ///
    /// if add_result.is_none() {
    ///     panic!("Overflow happened");
    /// }
    /// ```
    pub fn add_video_plays(&mut self, param: u32) -> Option<u32> {
        match self.video_plays.checked_add(param) {
            Some(result) => {
                self.video_plays = result;

                Some(result)
            }

            None => None,
        }
    }
    /// *Try to add an integer to the ad_impressions param*
    ///
    /// ---
    ///
    /// Note: Method is using the `checked_add()` method, to check for overflow.
    /// Returning None should signal an error.
    ///
    /// ---
    ///
    /// ## Arguments
    ///
    /// - `param` - Integer to append
    ///
    /// ## Example
    ///
    /// ```
    /// let owner_usage = OwnerUsage::default();
    /// let add_result = owner_usage.add_ad_impressions(20);
    ///
    /// if add_result.is_none() {
    ///     panic!("Overflow happened");
    /// }
    /// ```
    ///
    pub fn add_ad_impressions(&mut self, param: u32) -> Option<u32> {
        match self.ad_impressions.checked_add(param) {
            Some(result) => {
                self.ad_impressions = result;

                Some(result)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add_to_multiple_parameters() {
        let mut owner_usage = OwnerUsage::default();

        assert_eq!(owner_usage.add_video_plays(10), Some(10));
        assert_eq!(owner_usage.add_ad_impressions(20), Some(20));
        assert_eq!(owner_usage.get_video_plays(), 10);
        assert_eq!(owner_usage.get_ad_impressions(), 20);
    }

    #[test]
    fn adding_to_multiple_parameters_should_overflow() {
        let mut owner_usage = OwnerUsage::default();

        assert_eq!(owner_usage.add_video_plays(u32::MAX), Some(u32::MAX));
        assert_eq!(owner_usage.get_video_plays(), u32::MAX);
        assert_eq!(owner_usage.add_video_plays(1), None);
        assert_eq!(owner_usage.add_ad_impressions(u32::MAX), Some(u32::MAX));
        assert_eq!(owner_usage.get_ad_impressions(), u32::MAX);
        assert_eq!(owner_usage.add_ad_impressions(1), None);
    }
}
