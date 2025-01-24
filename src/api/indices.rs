use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize, Default, Clone)]
pub struct IndicesRequest {
    pub code: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct IndicesResponse {
    pub indices: Vec<IndexData>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct IndexData {
    pub Close: f64,
    pub Code: String,
    pub Date: String,
    pub High: f64,
    pub Low: f64,
    pub Open: f64,
}

impl ApiEndpoint for IndicesRequest {
    type Response = IndicesResponse;
    type ResData = IndexData;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/indices";
    
    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.indices
	}
}
