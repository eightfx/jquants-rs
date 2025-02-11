---
author: eight
---

# これは何？

J-Quants APIの非公式ラッパーです。

# 使い方

## インストール

``` shell
cargo add jquants_rs
```

## 使い方

対応するエンドポイントのRequest構造体に必要事項を入力して

### インスタンス生成と認証

``` rust
let mut client = HttpClient::new();
client.set_id_token("your_id_token".to_string());
```

### リクエスト送信

対応するエンドポイントのRequest構造体に必要事項を入力して
`client.send_request(request).await` でリクエストを送信します。

## 例

下記は

1.  メールアドレスとパスワードを使ってリフレッシュトークンを取得
2.  リフレッシュトークンを使ってIDトークンを取得
3.  IDトークンを使って各エンドポイントを叩く

という流れを示しています。

``` rust
let user_id = "your_user_id".to_string();
let user_password = "your_password".to_string();

let mut client = HttpClient::new();

// リフレッシュトークン作成
let login_request = token_auth_user::AuthUserRequest {
    mailaddress: user_id,
    password: user_password,
};
let login_response: Result<token_auth_user::AuthUserResponse, _> =
    client.send_request(login_request).await;
let refresh_token = login_response.unwrap().refreshToken;

// IDトークン作成
let id_token_request = token_auth_refresh::AuthRefreshRequest {
    refreshtoken: refresh_token,
};
let id_token_response: Result<token_auth_refresh::AuthRefreshResponse, _> =
    client.send_request(id_token_request).await;
client.set_id_token(id_token_response.unwrap().idToken);

// 上場銘柄一覧
let stock_list_request = api::listed_info::ListedInfoRequest::builder().code("13010".to_string()).build();
let stock_list_response: Result<api::listed_info::ListedInfoResponse, _> =
client.send_request(stock_list_request).await;
println!("{:?}", stock_list_response);

// 日足データ取得
let daily_query = api::prices_daily_quotes::DailyQuotesRequest::builder().code("13010".to_string()).date("20240605".to_string()).build();
let res = client.send_request(daily_query).await;
println!("{:?}", res);

// 投資部門別情報
let query = api::markets_investment_sector::InvestmentSectorRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 信用取引週末残高
let query = api::markets_weekly_margin_interest::WeeklyMarginInterestRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 業種別空売り残高
let query = api::markets_short_selling::ShortSellingRequest::builder().sector33code("0050".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 取引カレンダー
let query = api::markets_trading_calendar::TradingCalendarRequest::builder().holidaydivision("1".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 指数四本足
let query = api::indices::IndicesRequest::builder().code("13010".to_string()).from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// // TOPIX
let query = api::indices_topix::TopixRequest::builder().from("20240324".to_string()).to("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 財務情報
let query = api::fins_statements::StatementsRequest::builder().code("86970".to_string()).date("20230130".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);

// 決算発表日
let query = api::fins_announcement::AnnouncementRequest::builder().build();
let res = client.send_request(query).await;
println!("{:?}", res);

// オプションデータ
let query = api::option_index_option::IndexOptionRequest::builder().date("20240601".to_string()).build();
let res = client.send_request(query).await;
println!("{:?}", res);
```

# 対応済みAPI

- [x] /token/auth<sub>user</sub>
- [x] /token/auth<sub>refresh</sub>
- [x] /listed/info
- [x] /prices/daily<sub>quotes</sub>
- [ ] /prices/prices<sub>am</sub>
- [x] /markets/trades<sub>spec</sub>
- [x] /markets/weekly<sub>margininterest</sub>
- [x] /markets/short<sub>selling</sub>
- [ ] /markets/breakdown
- [x] /markets/trading<sub>calendar</sub>
- [x] /indices
- [x] /indices/topix
- [x] /fins/statements
- [ ] /fins/fs<sub>details</sub>
- [ ] /fins/dividend
- [x] /fins/announcement
- [x] /option/index<sub>option</sub>
