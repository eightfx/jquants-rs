use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder, Default, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct ListedInfoRequest {
    pub code: Option<String>,
    pub date: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct StockInfo {
    pub Code: String,
    pub CompanyName: String,
    pub CompanyNameEnglish: String,
    pub Date: String,
    pub MarginCode: Option<String>,
    pub MarginCodeName: Option<String>,
    pub MarketCode: String,
    pub MarketCodeName: String,
    pub ScaleCategory: String,
    pub Sector17Code: String,
    pub Sector17CodeName: String,
    pub Sector33Code: String,
    pub Sector33CodeName: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListedInfoResponse {
    pub info: Vec<StockInfo>,
}

impl ApiEndpoint for ListedInfoRequest {
    type Response = ListedInfoResponse;
    type ResData = StockInfo;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/listed/info";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.info
	}
}
