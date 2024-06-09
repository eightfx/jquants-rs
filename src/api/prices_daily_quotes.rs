use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize)]
pub struct DailyQuotesRequest {
    #[builder(setter(!strip_option))]
    pub code: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct DailyQuotesResponse {
    pub daily_quotes: Vec<DailyQuote>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct DailyQuote {
    pub Date: String,
    pub Code: String,
    pub Open: f64,
    pub High: f64,
    pub Low: f64,
    pub Close: f64,
    pub UpperLimit: String,
    pub LowerLimit: String,
    pub Volume: f64,
    pub TurnoverValue: f64,
    pub AdjustmentFactor: f64,
    pub AdjustmentOpen: f64,
    pub AdjustmentHigh: f64,
    pub AdjustmentLow: f64,
    pub AdjustmentClose: f64,
    pub AdjustmentVolume: f64,
    pub MorningOpen: Option<f64>,
    pub MorningHigh: Option<f64>,
    pub MorningLow: Option<f64>,
    pub MorningClose: Option<f64>,
    pub MorningUpperLimit: Option<String>,
    pub MorningLowerLimit: Option<String>,
    pub MorningVolume: Option<f64>,
    pub MorningTurnoverValue: Option<f64>,
    pub MorningAdjustmentOpen: Option<f64>,
    pub MorningAdjustmentHigh: Option<f64>,
    pub MorningAdjustmentLow: Option<f64>,
    pub MorningAdjustmentClose: Option<f64>,
    pub MorningAdjustmentVolume: Option<f64>,
    pub AfternoonOpen: Option<f64>,
    pub AfternoonHigh: Option<f64>,
    pub AfternoonLow: Option<f64>,
    pub AfternoonClose: Option<f64>,
    pub AfternoonUpperLimit: Option<String>,
    pub AfternoonLowerLimit: Option<String>,
    pub AfternoonVolume: Option<f64>,
    pub AfternoonTurnoverValue: Option<f64>,
    pub AfternoonAdjustmentOpen: Option<f64>,
    pub AfternoonAdjustmentHigh: Option<f64>,
    pub AfternoonAdjustmentLow: Option<f64>,
    pub AfternoonAdjustmentClose: Option<f64>,
    pub AfternoonAdjustmentVolume: Option<f64>,
}

impl ApiEndpoint for DailyQuotesRequest {
    type Response = DailyQuotesResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/prices/daily_quotes";
}
