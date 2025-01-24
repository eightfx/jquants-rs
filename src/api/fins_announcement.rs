use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize, Default, Clone)]
pub struct AnnouncementRequest {
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct AnnouncementResponse {
    pub announcement: Vec<Announcement>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct Announcement {
    pub Code: String,
    pub CompanyName: String,
    pub Date: String,
    pub FiscalQuarter: String,
    pub FiscalYear: String,
    pub Section: String,
    pub SectorName: String,
}

impl ApiEndpoint for AnnouncementRequest {
    type Response = AnnouncementResponse;
    type ResData = Announcement;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/fins/announcement";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.announcement
	}
}
