use std::future::Future;
use std::pin::Pin;

use crate::client::ApiClient;
use crate::models::{ApiCredentials, ApiParams, BaseClient, Endpoint};
use crate::request::api_request;
use crate::{
    client_defs, nullary_method_defs, nullary_method_impls,
    nullary_private_method_impls, nullary_public_method_impls,
    unary_method_defs, unary_method_impls, unary_private_method_impls,
    unary_public_method_impls,
};

/// Client for the Kraken public API. It implements all the methods under the
/// [`KrakenPublicEndpoints`] trait.
pub type KrakenPublicApiClient = ApiClient<KrakenPublicClient>;
/// Client for the Kraken private and public APIs. It implements all the methods
/// under the [`KrakenPublicEndpoints`] and [`KrakenPrivateApiClient`] traits.
pub type KrakenPrivateApiClient = ApiClient<KrakenPrivateClient>;

/// A collection of all the public endpoints of the [Kraken
/// API](https://docs.kraken.com/rest/#tag/Market-Data).
pub trait KrakenPublicEndpoints {
    nullary_method_defs! {
        time,
        system_status,
    }

    unary_method_defs! {
        assets,
        asset_pairs,
        ticker,
        ohlc,
        depth,
        trades,
        spread,
    }
}

/// A collection of all the private endpoints of the [Kraken
/// API](https://docs.kraken.com/rest/#tag/User-Data).
pub trait KrakenPrivateEndpoints {
    nullary_method_defs! {
        balance,
    }

    unary_method_defs! {
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
        add_export,
        export_status,
        retrieve_export,
        remove_export,
        add_order,
        cancel_order,
        cancel_all,
        cancel_all_after,
        deposit_methods,
        deposit_addresses,
        deposit_status,
        withdraw_info,
        withdraw,
        withdraw_status,
        withdraw_cancel,
        wallet_transfer,
        get_websockets_token,
    }
}

#[derive(Debug)]
pub struct KrakenPublicClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_params: ApiParams,
}

impl BaseClient for KrakenPublicClient {}

impl KrakenPublicClient {
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

pub struct KrakenPrivateClient {
    pub(crate) http: reqwest::Client,
    pub(crate) api_params: ApiParams,
    pub(crate) api_credentials: ApiCredentials,
}

impl BaseClient for KrakenPrivateClient {}

impl KrakenPrivateClient {
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

client_defs!(
    KrakenPublicClient,
    KrakenPublicApiClient,
    KrakenPrivateClient,
    KrakenPrivateApiClient,
    "https://api.kraken.com",
    "0"
);

impl KrakenPublicEndpoints for KrakenPublicApiClient {
    nullary_public_method_impls! {
        time: "Time",
        system_status: "SystemStatus",
    }

    unary_public_method_impls! {
        assets: "Assets",
        asset_pairs: "AssetPairs",
        ticker: "Ticker",
        ohlc: "OHLC",
        depth: "Depth",
        trades: "Trades",
        spread: "Spread",
    }
}

impl KrakenPublicEndpoints for KrakenPrivateApiClient {
    nullary_public_method_impls! {
        time: "Time",
        system_status: "SystemStatus",
    }

    unary_public_method_impls! {
        assets: "Assets",
        asset_pairs: "AssetPairs",
        ticker: "Ticker",
        depth: "Depth",
        trades: "Trades",
        spread: "Spread",
        ohlc: "OHLC",
    }
}

impl KrakenPrivateEndpoints for KrakenPrivateApiClient {
    nullary_private_method_impls! {
        balance: "Balance",
    }

    unary_private_method_impls! {
        trade_balance: "TradeBalance",
        open_orders: "OpenOrders",
        closed_orders: "ClosedOrders",
        query_orders: "QueryOrders",
        trades_history: "TradesHistory",
        query_trades: "QueryTrades",
        open_positions: "OpenPositions",
        ledgers: "Ledgers",
        query_ledgers: "QueryLedgers",
        trade_volume: "TradeVolume",
        add_export: "AddExport",
        export_status: "ExportStatus",
        retrieve_export: "RetrieveExport",
        remove_export: "RemoveExport",
        add_order: "AddOrder",
        cancel_order: "CancelOrder",
        cancel_all: "CancelAll",
        cancel_all_after: "CancelAllOrdersAfter",
        deposit_methods: "DepositMethods",
        deposit_addresses: "DepositAddresses",
        deposit_status: "DepositStatus",
        withdraw_info: "WithdrawInfo",
        withdraw: "Withdraw",
        withdraw_status: "WithdrawStatus",
        withdraw_cancel: "WithdrawCancel",
        wallet_transfer: "WalletTransfer",
        get_websockets_token: "GetWebsocketsToken",
    }
}
