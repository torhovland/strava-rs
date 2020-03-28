//! Athlete attempts at a segment
use super::api::{v3, AccessToken, Paginated, ResourceState};
use super::athletes::Athlete;
use super::error::Result;
use super::http::get;
use super::segments::Segment;
use serde::Deserialize;

/// An athlete's attempt at a segment (the portion of a ride that covers a segment)
///
/// Available in summary and detail representations, but they are the same at this time.
///
/// http://strava.github.io/api/v3/efforts/#retrieve
#[derive(Debug, Deserialize)]
pub struct SegmentEffort {
    id: i64,
    resource_state: ResourceState,
    name: String,
    // TODO activity: Activity, // Meta representation only
    athlete: Athlete, // Meta representation only
    elapsed_time: u32,
    moving_time: u32,
    start_date: String,
    start_date_local: String,
    distance: f32,
    start_index: u32,
    end_index: u32,
    average_cadence: f32,
    average_watts: f32,
    device_watts: bool,
    average_heartrate: f32,
    max_heartrate: f32,
    segment: Segment, // Summary representation
    kom_rank: Option<u8>,
    pr_rank: Option<u8>,
}

impl SegmentEffort {
    /// List efforts for the given segment ID
    ///
    /// http://strava.github.io/api/v3/segments/#efforts
    ///
    /// TODO support filtering by athlete
    /// TODO support filtering by date range
    pub async fn list_for_segment(
        token: &AccessToken,
        id: u32,
    ) -> Result<Paginated<SegmentEffort>> {
        let url = v3(Some(token), format!("segments/{}/all_efforts", id));
        let efforts = get::<Vec<SegmentEffort>>(&url[..]).await?;
        Ok(Paginated::new(url, efforts))
    }
}

#[cfg(feature = "api_test")]
#[cfg(test)]
mod api_tests {
    use super::SegmentEffort;
    use api::AccessToken;

    #[test]
    fn get_efforts_for_segment() {
        let token = AccessToken::new_from_env().unwrap();
        let pager = SegmentEffort::list_for_segment(&token, 646257).unwrap();
        println!("{:?}", pager);
    }
}
