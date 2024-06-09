use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize)]
pub struct AnnouncementRequest {
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct AnnouncementResponse {
    pub announcement: Vec<Announcement>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct Announcement {
    pub Date: String,
    pub Code: String,
    pub CompanyName: String,
    pub FiscalYear: String,
    pub SectorName: String,
    pub FiscalQuarter: String,
    pub Section: String,
}

impl ApiEndpoint for AnnouncementRequest {
    type Response = AnnouncementResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/fins/announcement";
}
