use std::future::Future;
use std::pin::Pin;

use crate::models::{ApiCredentials, ApiParams, Endpoint};
use crate::request::api_request;

macro_rules! method_defs {
    ( $($func:ident),* $(,)? ) => {
        $(
            fn $func<'a, 'async_trait>(&'a self, params: String) ->
                Pin<Box<dyn Future<Output = reqwest::Result<String>> + Send + 'async_trait>>
                where 'a: 'async_trait, Self: Sync + 'async_trait;
        )*
    };
}

pub trait PublicMethods {
    method_defs! {
        time,
        assets,
        asset_pairs,
        ticker,
        depth,
        trades,
        spread,
        ohlc,
    }
}

pub trait PrivateMethods {
    method_defs! {
        balance,
        trade_balance,
        open_orders,
        closed_orders,
        query_orders,
        trades_history,
        query_trades,
        open_positions,
        ledgers,
        query_ledgers,
        trade_volume,
        add_order,
        cancel_order,
        deposit_methods,
        deposit_addresses,
        deposit_status,
        withdraw_info,
        withdraw,
        withdraw_status,
        withdraw_cancel,
        get_websockets_token,
    }
}

pub trait BaseClient {}

#[derive(Debug, Default)]
pub struct PublicClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_params: ApiParams,
}

impl BaseClient for PublicClient {}

impl PublicClient {
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

pub struct PrivateClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_params: ApiParams,
    pub(crate) api_credentials: ApiCredentials,
}

impl BaseClient for PrivateClient {}

impl PrivateClient {
    pub fn new(
        http: reqwest::Client,
        api_params: ApiParams,
        api_credentials: ApiCredentials,
    ) -> Self {
        Self {
            http,
            api_params,
            api_credentials,
        }
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

    pub async fn private_api_request(
        &self,
        endpoint: &'static str,
        body: String,
    ) -> reqwest::Result<String> {
        api_request(
            &self.http,
            &self.api_params,
            Endpoint::Private(endpoint, self.api_credentials.clone()),
            &body,
        )
        .await
    }
}
