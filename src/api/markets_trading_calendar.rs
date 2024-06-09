use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize)]
pub struct TradingCalendarRequest {
    pub holidaydivision: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct TradingCalendarResponse {
    pub trading_calendar: Vec<TradingCalendar>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct TradingCalendar {
    pub Date: String,
    pub HolidayDivision: String,
}

impl ApiEndpoint for TradingCalendarRequest {
    type Response = TradingCalendarResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/markets/trading_calendar";
}
