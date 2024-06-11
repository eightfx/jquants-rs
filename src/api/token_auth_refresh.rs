use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize, Default, Clone)]
pub struct AuthRefreshRequest {
    pub refreshtoken: String,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct AuthRefreshResponse {
    pub idToken: String,
}

impl ApiEndpoint for AuthRefreshRequest {
    type Response = AuthRefreshResponse;
    const METHOD: ApiMethod = ApiMethod::POST;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/token/auth_refresh";
}
