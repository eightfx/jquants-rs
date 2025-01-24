use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize, Default, Clone)]
pub struct TopixRequest {
    pub from: Option<String>,
    pub to: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct TopixResponse {
    pub topix: Vec<TopixData>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct TopixData {
    pub Close: f64,
    pub Date: String,
    pub High: f64,
    pub Low: f64,
    pub Open: f64,
}

impl ApiEndpoint for TopixRequest {
    type Response = TopixResponse;
    type ResData = TopixData;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/indices/topix";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.topix
	}
}
