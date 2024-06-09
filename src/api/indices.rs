use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize)]
pub struct IndicesRequest {
    pub code: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct IndicesResponse {
    pub indices: Vec<IndexData>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct IndexData {
    pub Date: String,
    pub Code: String,
    pub Open: f64,
    pub High: f64,
    pub Low: f64,
    pub Close: f64,
}

impl ApiEndpoint for IndicesRequest {
    type Response = IndicesResponse;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/indices";
}
