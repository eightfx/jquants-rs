use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
pub mod api;

pub mod prelude {
    pub use super::api::*;
    // このファイルで定義される全てのstructなどもここに追加する
    pub use super::{ApiClient, ApiEndpoint, ApiInclude, ApiMethod, HttpClient};


}


// APIエンドポイントのトレイト定義
pub trait ApiEndpoint: Serialize {
    type Response: DeserializeOwned;
    const METHOD: ApiMethod;
    const INCLUDE: ApiInclude;
    const ENDPOINT: &'static str;

    // fn endpoint() -> &'static str;

    fn parse_response(response: &str) -> Result<Self::Response, String> {
        serde_json::from_str(response).map_err(|e| e.to_string())
    }
}

#[derive(PartialEq)]
pub enum ApiMethod {
    GET,
    POST,
}

#[derive(PartialEq)]
pub enum ApiInclude {
    BODY,
    QUERY,
}

// エンドポイントごとのリクエストとレスポンスの定義
// APIクライアントのトレイト定義
// #[async_trait]
#[trait_variant::make(IntFactory: Send)]
pub trait ApiClient {
    fn base_url(&self) -> &str;

    async fn send_request<E>(&self, request: E) -> Result<E::Response, String>
	where E: ApiEndpoint + Send + 'static;

}

// 具体的なクライアントの実装
pub struct HttpClient {
    base_url: String,
    client: Client,
    id_token: Option<String>,
}

impl HttpClient {
    pub fn new() -> Self {
        HttpClient {
            base_url: "https://api.jquants.com".to_string(),
            client: Client::new(),
            id_token: None,
        }
    }

    pub fn set_id_token(&mut self, id_token: String) {
		self.id_token = Some(id_token);
	}
}

impl ApiClient for HttpClient {
    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn send_request<E>(&self, request: E) -> Result<E::Response, String>
	where E: ApiEndpoint + Send + 'static
    {
        let url = format!("{}{}", self.base_url(), E::ENDPOINT);

        // ユーザー認証の場合のみbodyに含める
        let mut request_builder = if E::METHOD == ApiMethod::POST {
            self.client.post(&url)
        } else {
            self.client.get(&url)
        };

        request_builder = if E::INCLUDE == ApiInclude::BODY {
            request_builder.json(&request)
        } else {
            request_builder.query(&request)
        };

        // id_tokenがあればリクエストヘッダに含める
        if let Some(id_token) = &self.id_token {
            request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {}", id_token));
        }

        let response = request_builder.send();

        match response.await {
            Ok(resp) => {
                if resp.status().is_success() {
                    let response_text = resp.text().await.map_err(|e| e.to_string())?;
                    E::parse_response(&response_text)
                } else {
                    // let error_text = resp.text().map_err.await(|e| e.to_string())?;
                    Err(format!(
                        "Error {}: {}",
                        resp.status(),
                        resp.text().await.map_err(|e| e.to_string())?
                    ))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_login() {
        let mut client = HttpClient::new();

        let login_request = api::token_auth_user::AuthUserRequest {
            mailaddress: "test@test.test".to_string(),
            password: "testtest".to_string(),
        };

        let login_response: Result<api::token_auth_user::AuthUserResponse, _> =
            client.send_request(login_request).await;
        let refresh_token = login_response.unwrap().refreshToken;

        let refresh_token_request = api::token_auth_refresh::AuthRefreshRequest {
            refreshtoken: refresh_token,
        };
        let refresh_response: Result<api::token_auth_refresh::AuthRefreshResponse, _> =
            client.send_request(refresh_token_request).await;

        client.set_id_token(refresh_response.unwrap().idToken);

        // 上場銘柄一覧
        // let stock_list_request = api::listed_info::ListedInfoRequest::builder().code("13010".to_string()).build();
        // let stock_list_response: Result<api::listed_info::ListedInfoResponse, _> =
        // client.send_request(stock_list_request).await;
        // println!("{:?}", stock_list_response);

        // 日足データ取得
        // let daily_query = api::prices_daily_quotes::DailyQuotesRequest::builder().code("13010".to_string()).date("20240605".to_string()).build();
        // let res = client.send_request(daily_query).await;
        // println!("{:?}", res);

        // 投資部門別情報
        // let query = api::markets_investment_sector::InvestmentSectorRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 信用取引週末残高
        // let query = api::markets_weekly_margin_interest::WeeklyMarginInterestRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 業種別空売り残高
        // let query = api::markets_short_selling::ShortSellingRequest::builder().sector33code("0050".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 取引カレンダー
        // let query = api::markets_trading_calendar::TradingCalendarRequest::builder().holidaydivision("1".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 指数四本足
        // let query = api::indices::IndicesRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // // TOPIX
        // let query = api::indices_topix::TopixRequest::builder().from("20240324".to_string()).to("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 財務情報
        // let query = api::fins_statements::StatementsRequest::builder().code("86970".to_string()).date("20230130".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // 決算発表日
        // let query = api::fins_announcement::AnnouncementRequest::builder().build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // オプションデータ
        // let query = api::option_index_option::IndexOptionRequest::builder().date("20240601".to_string()).build();
        // let res = client.send_request(query).await;
        // println!("{:?}", res)

        // assert!(login_response.is_ok());
        // if let Ok(response) = login_response {
        //     assert_eq!(response.refreshToken, "mock_refresh_token");
        // }
    }
}
