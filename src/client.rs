use std::future::Future;
use std::pin::Pin;

use crate::methods::{
    BaseClient, PrivateClient, PrivateMethods, PublicClient, PublicMethods,
};
use crate::models::{ApiCredentials, ApiParams};

pub type PublicApiClient = ApiClient<PublicClient>;
pub type PrivateApiClient = ApiClient<PrivateClient>;

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

    pub fn default(api_credentials: ApiCredentials) -> Self {
        Self {
            client: PrivateClient::new(
                reqwest::Client::default(),
                ApiParams::default(),
                api_credentials,
            ),
        }
    }
}

macro_rules! method_impls {
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

macro_rules! public_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        method_impls!(public_api_request, $($func: $name,)*);
    };
}

macro_rules! private_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        method_impls!(private_api_request, $($func: $name,)*);
    };
}

impl PublicMethods for PublicApiClient {
    public_method_impls! {
        time: "Time",
        assets: "Assets",
        asset_pairs: "AssetPairs",
        ticker: "Ticker",
        depth: "Depth",
        trades: "Trades",
        spread: "Spread",
        ohlc: "OHLC",
    }
}

impl PublicMethods for PrivateApiClient {
    public_method_impls! {
        time: "Time",
        assets: "Assets",
        asset_pairs: "AssetPairs",
        ticker: "Ticker",
        depth: "Depth",
        trades: "Trades",
        spread: "Spread",
        ohlc: "OHLC",
    }
}

impl PrivateMethods for PrivateApiClient {
    private_method_impls! {
        balance: "Balance",
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
        add_order: "AddOrder",
        cancel_order: "CancelOrder",
        deposit_methods: "DepositMethods",
        deposit_addresses: "DepositAddresses",
        deposit_status: "DepositStatus",
        withdraw_info: "WithdrawInfo",
        withdraw: "Withdraw",
        withdraw_status: "WithdrawStatus",
        withdraw_cancel: "WithdrawCancel",
        get_websockets_token: "GetWebsocketsToken",
    }
}
