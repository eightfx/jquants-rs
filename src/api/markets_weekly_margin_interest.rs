use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize)]
pub struct WeeklyMarginInterestRequest {
    pub code: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct WeeklyMarginInterestResponse {
    pub weekly_margin_interest: Vec<WeeklyMarginInterest>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct WeeklyMarginInterest {
    pub Date: String,
    pub Code: String,
    pub ShortMarginTradeVolume: f64,
    pub LongMarginTradeVolume: f64,
    pub ShortNegotiableMarginTradeVolume: f64,
    pub LongNegotiableMarginTradeVolume: f64,
    pub ShortStandardizedMarginTradeVolume: f64,
    pub LongStandardizedMarginTradeVolume: f64,
    pub IssueType: String,
}

impl ApiEndpoint for WeeklyMarginInterestRequest {
    type Response = WeeklyMarginInterestResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/markets/weekly_margin_interest";
}
