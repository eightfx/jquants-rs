use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, Clone, TypedBuilder, Default)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct ShortSellingRequest {
    pub sector33code: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct ShortSellingResponse {
    pub short_selling: Vec<ShortSelling>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct ShortSelling {
    pub Date: String,
    pub Sector33Code: String,
    pub SellingExcludingShortSellingTurnoverValue: f64,
    pub ShortSellingWithRestrictionsTurnoverValue: f64,
    pub ShortSellingWithoutRestrictionsTurnoverValue: f64,
}

impl ApiEndpoint for ShortSellingRequest {
    type Response = ShortSellingResponse;
    type ResData = ShortSelling;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/markets/short_selling";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.short_selling
	}
}
