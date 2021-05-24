use std::future::Future;
use std::pin::Pin;

use crate::client::ApiClient;
use crate::models::{ApiParams, BaseClient, Endpoint};
use crate::request::api_request;
use crate::{
    client_defs, nullary_method_defs, nullary_method_impls,
    nullary_public_method_impls, unary_method_defs, unary_method_impls,
    unary_public_method_impls,
};

/// Client for the Deribit public API. It implements all the methods under the
/// [`DeribitPublicEndpoints`] trait.
pub type DeribitPublicApiClient = ApiClient<DeribitPublicClient>;

/// A collection of all the public endpoints of the [Deribit
/// API](https://docs.Deribit.com/rest/#tag/Market-Data).
pub trait DeribitPublicEndpoints {
    nullary_method_defs! {
        get_time,
        get_currencies,
        get_index_price_names,
    }

    unary_method_defs! {
        hello,
        test,
        get_announcements,
        get_book_summary_by_currency,
        get_book_summary_by_instrument,
        get_contract_size,
        get_funding_chart_data,
        get_funding_rate_history,
        get_funding_rate_value,
        get_historical_volatility,
        get_index_price,
        get_instrument,
        get_instruments,
        get_last_settlements_by_currency,
        get_last_settlements_by_instrument,
        get_last_trades_by_currency,
        get_last_trades_by_currency_and_time,
        get_last_trades_by_instrument,
        get_last_trades_by_instrument_and_time,
        get_mark_price_history,
        get_order_book,
        get_trade_volumes,
        get_tradingview_chart_data,
        get_volatility_index_data,
        ticker,
    }
}

#[derive(Debug)]
pub struct DeribitPublicClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_params: ApiParams,
}

impl BaseClient for DeribitPublicClient {}

impl DeribitPublicClient {
    pub fn new(http: reqwest::Client, api_params: ApiParams) -> Self {
        Self { http, api_params }
    }

    pub async fn public_api_request(
        &self,
        endpoint: &'static str,
        body: String,
    ) -> reqwest::Result<String> {
        api_request(
            &self.http,
            &self.api_params,
            Endpoint::Public(endpoint),
            &body,
        )
        .await
    }
}

client_defs!(
    DeribitPublicClient,
    DeribitPublicApiClient,
    "https://deribit.com/api",
    "v2"
);

impl DeribitPublicEndpoints for DeribitPublicApiClient {
    nullary_public_method_impls! {
        get_time: "get_time",
        get_currencies: "get_currencies",
        get_index_price_names: "get_index_price_names",
    }

    unary_public_method_impls! {
        hello: "hello",
        test: "test",
        get_announcements: "get_announcements",
        get_book_summary_by_currency: "get_book_summary_by_currency",
        get_book_summary_by_instrument: "get_book_summary_by_instrument",
        get_contract_size: "get_contract_size",
        get_funding_chart_data: "get_funding_chart_data",
        get_funding_rate_history: "get_funding_rate_history",
        get_funding_rate_value: "get_funding_rate_value",
        get_historical_volatility: "get_historical_volatility",
        get_index_price: "get_index_price",
        get_instrument: "get_instrument",
        get_instruments: "get_instruments",
        get_last_settlements_by_currency: "get_last_settlements_by_currency",
        get_last_settlements_by_instrument: "get_last_settlements_by_instrument",
        get_last_trades_by_currency: "get_last_trades_by_currency",
        get_last_trades_by_currency_and_time: "get_last_trades_by_currency_and_time",
        get_last_trades_by_instrument: "get_last_trades_by_instrument",
        get_last_trades_by_instrument_and_time: "get_last_trades_by_instrument_and_time",
        get_mark_price_history: "get_mark_price_history",
        get_order_book: "get_order_book",
        get_trade_volumes: "get_trade_volumes",
        get_tradingview_chart_data: "get_tradingview_chart_data",
        get_volatility_index_data: "get_volatility_index_data",
        ticker: "ticker",
    }
}
