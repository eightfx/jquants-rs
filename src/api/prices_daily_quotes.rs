use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder, Default, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct DailyQuotesRequest {
    #[builder(setter(!strip_option))]
    pub code: String,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DailyQuotesResponse {
    pub daily_quotes: Vec<DailyQuote>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DailyQuote {
    pub AdjustmentClose: Option<f64>,
    pub AdjustmentFactor: Option<f64>,
    pub AdjustmentHigh: Option<f64>,
    pub AdjustmentLow: Option<f64>,
    pub AdjustmentOpen: Option<f64>,
    pub AdjustmentVolume: Option<f64>,
    pub AfternoonAdjustmentClose: Option<f64>,
    pub AfternoonAdjustmentHigh: Option<f64>,
    pub AfternoonAdjustmentLow: Option<f64>,
    pub AfternoonAdjustmentOpen: Option<f64>,
    pub AfternoonAdjustmentVolume: Option<f64>,
    pub AfternoonClose: Option<f64>,
    pub AfternoonHigh: Option<f64>,
    pub AfternoonLow: Option<f64>,
    pub AfternoonLowerLimit: Option<String>,
    pub AfternoonOpen: Option<f64>,
    pub AfternoonTurnoverValue: Option<f64>,
    pub AfternoonUpperLimit: Option<String>,
    pub AfternoonVolume: Option<f64>,
    pub Close: Option<f64>,
    pub Code: Option<String>,
    pub Date: Option<String>,
    pub High: Option<f64>,
    pub Low: Option<f64>,
    pub LowerLimit: Option<String>,
    pub MorningAdjustmentClose: Option<f64>,
    pub MorningAdjustmentHigh: Option<f64>,
    pub MorningAdjustmentLow: Option<f64>,
    pub MorningAdjustmentOpen: Option<f64>,
    pub MorningAdjustmentVolume: Option<f64>,
    pub MorningClose: Option<f64>,
    pub MorningHigh: Option<f64>,
    pub MorningLow: Option<f64>,
    pub MorningLowerLimit: Option<String>,
    pub MorningOpen: Option<f64>,
    pub MorningTurnoverValue: Option<f64>,
    pub MorningUpperLimit: Option<String>,
    pub MorningVolume: Option<f64>,
    pub Open: Option<f64>,
    pub TurnoverValue: Option<f64>,
    pub UpperLimit: Option<String>,
    pub Volume: Option<f64>,
}

impl ApiEndpoint for DailyQuotesRequest {
    type Response = DailyQuotesResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/prices/daily_quotes";
}
