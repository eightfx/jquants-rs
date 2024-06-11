use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder, Default, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct IndexOptionRequest {
    #[builder(setter(!strip_option))]
    pub date: String,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct IndexOptionResponse {
    pub index_option: Vec<IndexOption>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct IndexOption {
    pub BaseVolatility: f64,
    pub Code: String,
    pub ContractMonth: String,
    pub Date: String,
    pub DaySessionClose: f64,
    pub DaySessionHigh: f64,
    pub DaySessionLow: f64,
    pub DaySessionOpen: f64,
    pub EmergencyMarginTriggerDivision: String,
    pub ImpliedVolatility: f64,
    pub InterestRate: f64,
    pub LastTradingDay: String,
    pub NightSessionClose: Option<f64>,
    pub NightSessionHigh: Option<f64>,
    pub NightSessionLow: Option<f64>,
    pub NightSessionOpen: Option<f64>,
    pub OpenInterest: f64,
    pub PutCallDivision: String,
    pub SettlementPrice: f64,
    pub SpecialQuotationDay: String,
    pub StrikePrice: f64,
    pub TheoreticalPrice: f64,
    pub TurnoverValue: f64,
    pub UnderlyingPrice: f64,
    pub Volume: f64,
    pub VolumeOnlyAuction: f64,
    pub WholeDayClose: f64,
    pub WholeDayHigh: f64,
    pub WholeDayLow: f64,
    pub WholeDayOpen: f64,
}

impl ApiEndpoint for IndexOptionRequest {
    type Response = IndexOptionResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/option/index_option";
}
