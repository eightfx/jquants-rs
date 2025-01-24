use crate::ApiEndpoint;
use crate::ApiInclude;
use crate::ApiMethod;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
#[builder(field_defaults(default, setter(strip_option)))]
#[derive(Serialize, Default, Clone)]
pub struct StatementsRequest {
    pub code: Option<String>,
    pub date: Option<String>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct StatementsResponse {
    pub statements: Vec<Statement>,
    pub pagination_key: Option<String>,
}

#[allow(warnings)]
#[derive(Debug, Deserialize, Clone, Default, Serialize)]
pub struct Statement {
    pub AverageNumberOfShares: Option<String>,
    pub BookValuePerShare: Option<String>,
    pub CashAndEquivalents: Option<String>,
    pub CashFlowsFromFinancingActivities: Option<String>,
    pub CashFlowsFromInvestingActivities: Option<String>,
    pub CashFlowsFromOperatingActivities: Option<String>,
    pub ChangesBasedOnRevisionsOfAccountingStandard: Option<String>,
    pub ChangesInAccountingEstimates: Option<String>,
    pub ChangesOtherThanOnesBasedOnRevisionsOfAccountingStandard: Option<String>,
    pub CurrentFiscalYearEndDate: String,
    pub CurrentFiscalYearStartDate: String,
    pub CurrentPeriodEndDate: String,
    pub CurrentPeriodStartDate: String,
    pub DilutedEarningsPerShare: Option<String>,
    pub DisclosedDate: String,
    pub DisclosedTime: String,
    pub DisclosureNumber: String,
    pub DistributionsPerUnit: Option<String>,
    pub EarningsPerShare: Option<String>,
    pub Equity: Option<String>,
    pub EquityToAssetRatio: Option<String>,
    pub ForecastDistributionsPerUnit: Option<String>,
    pub ForecastDividendPerShare1stQuarter: Option<String>,
    pub ForecastDividendPerShare2ndQuarter: Option<String>,
    pub ForecastDividendPerShare3rdQuarter: Option<String>,
    pub ForecastDividendPerShareAnnual: Option<String>,
    pub ForecastDividendPerShareFiscalYearEnd: Option<String>,
    pub ForecastEarningsPerShare2ndQuarter: Option<String>,
    pub ForecastEarningsPerShare: Option<String>,
    pub ForecastNetSales2ndQuarter: Option<String>,
    pub ForecastNetSales: Option<String>,
    pub ForecastNonConsolidatedEarningsPerShare2ndQuarter: Option<String>,
    pub ForecastNonConsolidatedEarningsPerShare: Option<String>,
    pub ForecastNonConsolidatedNetSales2ndQuarter: Option<String>,
    pub ForecastNonConsolidatedNetSales: Option<String>,
    pub ForecastNonConsolidatedOperatingProfit2ndQuarter: Option<String>,
    pub ForecastNonConsolidatedOperatingProfit: Option<String>,
    pub ForecastNonConsolidatedOrdinaryProfit2ndQuarter: Option<String>,
    pub ForecastNonConsolidatedOrdinaryProfit: Option<String>,
    pub ForecastNonConsolidatedProfit2ndQuarter: Option<String>,
    pub ForecastNonConsolidatedProfit: Option<String>,
    pub ForecastOperatingProfit2ndQuarter: Option<String>,
    pub ForecastOperatingProfit: Option<String>,
    pub ForecastOrdinaryProfit2ndQuarter: Option<String>,
    pub ForecastOrdinaryProfit: Option<String>,
    pub ForecastPayoutRatioAnnual: Option<String>,
    pub ForecastProfit2ndQuarter: Option<String>,
    pub ForecastProfit: Option<String>,
    pub ForecastTotalDividendPaidAnnual: Option<String>,
    pub LocalCode: String,
    pub MaterialChangesInSubsidiaries: Option<String>,
    pub NetSales: Option<String>,
    pub NextFiscalYearEndDate: Option<String>,
    pub NextFiscalYearStartDate: Option<String>,
    pub NextYearForecastDistributionsPerUnit: Option<String>,
    pub NextYearForecastDividendPerShare1stQuarter: Option<String>,
    pub NextYearForecastDividendPerShare2ndQuarter: Option<String>,
    pub NextYearForecastDividendPerShare3rdQuarter: Option<String>,
    pub NextYearForecastDividendPerShareAnnual: Option<String>,
    pub NextYearForecastDividendPerShareFiscalYearEnd: Option<String>,
    pub NextYearForecastEarningsPerShare2ndQuarter: Option<String>,
    pub NextYearForecastEarningsPerShare: Option<String>,
    pub NextYearForecastNetSales2ndQuarter: Option<String>,
    pub NextYearForecastNetSales: Option<String>,
    pub NextYearForecastNonConsolidatedEarningsPerShare2ndQuarter: Option<String>,
    pub NextYearForecastNonConsolidatedEarningsPerShare: Option<String>,
    pub NextYearForecastNonConsolidatedNetSales2ndQuarter: Option<String>,
    pub NextYearForecastNonConsolidatedNetSales: Option<String>,
    pub NextYearForecastNonConsolidatedOperatingProfit2ndQuarter: Option<String>,
    pub NextYearForecastNonConsolidatedOperatingProfit: Option<String>,
    pub NextYearForecastNonConsolidatedOrdinaryProfit2ndQuarter: Option<String>,
    pub NextYearForecastNonConsolidatedOrdinaryProfit: Option<String>,
    pub NextYearForecastNonConsolidatedProfit2ndQuarter: Option<String>,
    pub NextYearForecastNonConsolidatedProfit: Option<String>,
    pub NextYearForecastOperatingProfit2ndQuarter: Option<String>,
    pub NextYearForecastOperatingProfit: Option<String>,
    pub NextYearForecastOrdinaryProfit2ndQuarter: Option<String>,
    pub NextYearForecastOrdinaryProfit: Option<String>,
    pub NextYearForecastPayoutRatioAnnual: Option<String>,
    pub NextYearForecastProfit2ndQuarter: Option<String>,
    pub NextYearForecastProfit: Option<String>,
    pub NonConsolidatedBookValuePerShare: Option<String>,
    pub NonConsolidatedEarningsPerShare: Option<String>,
    pub NonConsolidatedEquity: Option<String>,
    pub NonConsolidatedEquityToAssetRatio: Option<String>,
    pub NonConsolidatedNetSales: Option<String>,
    pub NonConsolidatedOperatingProfit: Option<String>,
    pub NonConsolidatedOrdinaryProfit: Option<String>,
    pub NonConsolidatedProfit: Option<String>,
    pub NonConsolidatedTotalAssets: Option<String>,
    pub NumberOfIssuedAndOutstandingSharesAtTheEndOfFiscalYearIncludingTreasuryStock:Option<String>,
    pub NumberOfTreasuryStockAtTheEndOfFiscalYear: Option<String>,
    pub OperatingProfit: Option<String>,
    pub OrdinaryProfit: Option<String>,
    pub Profit: Option<String>,
    pub ResultDividendPerShare1stQuarter: Option<String>,
    pub ResultDividendPerShare2ndQuarter: Option<String>,
    pub ResultDividendPerShare3rdQuarter: Option<String>,
    pub ResultDividendPerShareAnnual: Option<String>,
    pub ResultDividendPerShareFiscalYearEnd: Option<String>,
    pub ResultPayoutRatioAnnual: Option<String>,
    pub ResultTotalDividendPaidAnnual: Option<String>,
    pub RetrospectiveRestatement: Option<String>,
    pub TotalAssets: Option<String>,
    pub TypeOfCurrentPeriod: String,
    pub TypeOfDocument: String,
}

impl ApiEndpoint for StatementsRequest {
    type Response = StatementsResponse;
    type ResData = Statement;
    const METHOD: ApiMethod = ApiMethod::GET;
    const INCLUDE: ApiInclude = ApiInclude::QUERY;
    const ENDPOINT: &'static str = "/v1/fins/statements";

    fn extract(response: Self::Response) -> Vec<Self::ResData> {
		response.statements
	}
}
