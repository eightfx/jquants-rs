use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Serialize, TypedBuilder, Clone, Default)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct TradesSpecRequest {
    pub section: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct TradesSpecResponse {
    pub trades_spec: Vec<TradeSpec>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct TradeSpec {
    pub BrokerageBalance: f64,
    pub BrokeragePurchases: f64,
    pub BrokerageSales: f64,
    pub BrokerageTotal: f64,
    pub BusinessCosBalance: f64,
    pub BusinessCosPurchases: f64,
    pub BusinessCosSales: f64,
    pub BusinessCosTotal: f64,
    pub CityBKsRegionalBKsEtcBalance: f64,
    pub CityBKsRegionalBKsEtcPurchases: f64,
    pub CityBKsRegionalBKsEtcSales: f64,
    pub CityBKsRegionalBKsEtcTotal: f64,
    pub EndDate: String,
    pub ForeignersBalance: f64,
    pub ForeignersPurchases: f64,
    pub ForeignersSales: f64,
    pub ForeignersTotal: f64,
    pub IndividualsBalance: f64,
    pub IndividualsPurchases: f64,
    pub IndividualsSales: f64,
    pub IndividualsTotal: f64,
    pub InsuranceCosBalance: f64,
    pub InsuranceCosPurchases: f64,
    pub InsuranceCosSales: f64,
    pub InsuranceCosTotal: f64,
    pub InvestmentTrustsBalance: f64,
    pub InvestmentTrustsPurchases: f64,
    pub InvestmentTrustsSales: f64,
    pub InvestmentTrustsTotal: f64,
    pub OtherCosBalance: f64,
    pub OtherCosPurchases: f64,
    pub OtherCosSales: f64,
    pub OtherCosTotal: f64,
    pub OtherFinancialInstitutionsBalance: f64,
    pub OtherFinancialInstitutionsPurchases: f64,
    pub OtherFinancialInstitutionsSales: f64,
    pub OtherFinancialInstitutionsTotal: f64,
    pub ProprietaryBalance: f64,
    pub ProprietaryPurchases: f64,
    pub ProprietarySales: f64,
    pub ProprietaryTotal: f64,
    pub PublishedDate: String,
    pub Section: String,
    pub SecuritiesCosBalance: f64,
    pub SecuritiesCosPurchases: f64,
    pub SecuritiesCosSales: f64,
    pub SecuritiesCosTotal: f64,
    pub StartDate: String,
    pub TotalBalance: f64,
    pub TotalPurchases: f64,
    pub TotalSales: f64,
    pub TotalTotal: f64,
    pub TrustBanksBalance: f64,
    pub TrustBanksPurchases: f64,
    pub TrustBanksSales: f64,
    pub TrustBanksTotal: f64,
}

impl ApiEndpoint for TradesSpecRequest {
    type Response = TradesSpecResponse;
    type ResData = TradeSpec;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/markets/trades_spec";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.trades_spec
	}
}
