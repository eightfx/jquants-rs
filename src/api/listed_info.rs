use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct ListedInfoRequest {
    pub code: Option<String>,
    pub date: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StockInfo {
    pub Date: String,
    pub Code: String,
    pub CompanyName: String,
    pub CompanyNameEnglish: String,
    pub Sector17Code: String,
    pub Sector17CodeName: String,
    pub Sector33Code: String,
    pub Sector33CodeName: String,
    pub ScaleCategory: String,
    pub MarketCode: String,
    pub MarketCodeName: String,
    pub MarginCode: String,
    pub MarginCodeName: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListedInfoResponse {
    pub info: Vec<StockInfo>,
}

impl ApiEndpoint for ListedInfoRequest {
    type Response = ListedInfoResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/listed/info";
}
