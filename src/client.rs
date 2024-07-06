use std::future::Future;
use std::pin::Pin;

use crate::endpoints::{
    BaseClient, PrivateClient, PrivateEndpoints, PublicClient, PublicEndpoints,
};
use crate::models::{ApiCredentials, ApiParams};

/// Client for the public API. It implements all the methods under the
/// [`PublicEndpoints`] trait.
pub type PublicApiClient = ApiClient<PublicClient>;
/// Client for the private and public APIs. It implements all the methods under
/// the [`PublicEndpoints`] and [`PrivateApiClient`] traits.
pub type PrivateApiClient = ApiClient<PrivateClient>;

/// Base client class for the API. It shouldn't be used directly, rather prefer
/// the type synonyms [`PublicApiClient`] and [`PrivateApiClient`].
pub struct ApiClient<C: BaseClient> {
    client: C,
}

impl PublicApiClient {
    pub fn new(http: reqwest::Client, api_params: ApiParams) -> Self {
        Self {
            client: PublicClient::new(http, api_params),
        }
    }

    pub fn default() -> Self {
        Self {
            client: PublicClient::default(),
        }
    }

    /// Consume the public client to get a private one, by supplying the API
    /// credentials.
    pub fn set_credentials(
        self,
        api_credentials: ApiCredentials,
    ) -> PrivateApiClient {
        ApiClient {
            client: PrivateClient::new(
                self.client.http,
                self.client.api_params,
                api_credentials,
            ),
        }
    }
}

impl PrivateApiClient {
    pub fn new(
        http: reqwest::Client,
        api_params: ApiParams,
        api_credentials: ApiCredentials,
    ) -> Self {
        Self {
            client: PrivateClient::new(http, api_params, api_credentials),
        }
    }

    pub fn default_with_credentials(api_credentials: ApiCredentials) -> Self {
        Self {
            client: PrivateClient::new(
                reqwest::Client::default(),
                ApiParams::default(),
                api_credentials,
            ),
        }
    }
}

macro_rules! nullary_method_impls {
    ( $api_request:ident, $($func:ident: $name:expr),* $(,)? ) => {
        $(
            fn $func<'a, 'async_trait>(&'a self) ->
                Pin<Box<dyn Future<Output = reqwest::Result<String>> + Send + 'async_trait>>
                where 'a: 'async_trait, Self: Sync + 'async_trait {
                Box::pin(self.client.$api_request($name, "".into()))
            }
        )*
    };
}

macro_rules! unary_method_impls {
    ( $api_request:ident, $($func:ident: $name:expr),* $(,)? ) => {
        $(
            fn $func<'a, 'async_trait>(&'a self, params: String) ->
                Pin<Box<dyn Future<Output = reqwest::Result<String>> + Send + 'async_trait>>
                where 'a: 'async_trait, Self: Sync + 'async_trait {
                Box::pin(self.client.$api_request($name, params))
            }
        )*
    };
}

macro_rules! nullary_public_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        nullary_method_impls!(public_api_request, $($func: $name,)*);
    };
}

macro_rules! unary_public_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        unary_method_impls!(public_api_request, $($func: $name,)*);
    };
}

macro_rules! nullary_private_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        nullary_method_impls!(private_api_request, $($func: $name,)*);
    };
}

macro_rules! unary_private_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        unary_method_impls!(private_api_request, $($func: $name,)*);
    };
}

impl PublicEndpoints for PublicApiClient {
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

impl PublicEndpoints for PrivateApiClient {
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

impl PrivateEndpoints for PrivateApiClient {
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
        edit_order: "EditOrder",
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
