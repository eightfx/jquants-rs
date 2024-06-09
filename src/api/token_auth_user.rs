use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder, Serialize)]
pub struct AuthUserRequest {
    pub mailaddress: String,
    pub password: String,
}

#[allow(warnings)]
#[derive(Debug, Deserialize)]
pub struct AuthUserResponse {
    pub refreshToken: String,
}

impl ApiEndpoint for AuthUserRequest {
    type Response = AuthUserResponse;
    const METHOD: ApiMethod = ApiMethod::POST;
    const INCLUDE: ApiInclude = ApiInclude::BODY;
    const ENDPOINT: &'static str = "/v1/token/auth_user";
}
