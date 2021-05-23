#[macro_export]
macro_rules! nullary_method_defs {
    ( $($func:ident),* $(,)? ) => {
        $(
            fn $func<'a, 'async_trait>(&'a self) ->
                Pin<Box<dyn Future<Output = reqwest::Result<String>> + Send + 'async_trait>>
                where 'a: 'async_trait, Self: Sync + 'async_trait;
        )*
    };
}

#[macro_export]
macro_rules! unary_method_defs {
    ( $($func:ident),* $(,)? ) => {
        $(
            fn $func<'a, 'async_trait>(&'a self, params: String) ->
                Pin<Box<dyn Future<Output = reqwest::Result<String>> + Send + 'async_trait>>
                where 'a: 'async_trait, Self: Sync + 'async_trait;
        )*
    };
}

#[macro_export]
macro_rules! client_defs {
    ($pub_client:ident, $pub_api:ident, $pri_client:ident, $pri_api:ident) => {
        impl $pub_api {
            pub fn new(http: reqwest::Client, api_params: ApiParams) -> Self {
                Self {
                    client: $pub_client::new(http, api_params),
                }
            }

            pub fn default() -> Self {
                Self {
                    client: $pub_client::default(),
                }
            }

            /// Consume the public client to get a private one, by supplying the
            /// API credentials.
            pub fn set_credentials(
                self,
                api_credentials: ApiCredentials,
            ) -> $pri_api {
                $pri_api {
                    client: $pri_client::new(
                        self.client.http,
                        self.client.api_params,
                        api_credentials,
                    ),
                }
            }
        }

        impl $pri_api {
            pub fn new(
                http: reqwest::Client,
                api_params: ApiParams,
                api_credentials: ApiCredentials,
            ) -> Self {
                Self {
                    client: $pri_client::new(
                        http,
                        api_params,
                        api_credentials,
                    ),
                }
            }

            pub fn default_with_credentials(
                api_credentials: ApiCredentials,
            ) -> Self {
                Self {
                    client: $pri_client::new(
                        reqwest::Client::default(),
                        ApiParams::default(),
                        api_credentials,
                    ),
                }
            }
        }
    };
}

#[macro_export]
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

#[macro_export]
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

#[macro_export]
macro_rules! nullary_public_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        nullary_method_impls!(public_api_request, $($func: $name,)*);
    };
}

#[macro_export]
macro_rules! unary_public_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        unary_method_impls!(public_api_request, $($func: $name,)*);
    };
}

#[macro_export]
macro_rules! nullary_private_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        nullary_method_impls!(private_api_request, $($func: $name,)*);
    };
}

#[macro_export]
macro_rules! unary_private_method_impls {
    ( $($func:ident: $name:expr),* $(,)? ) => {
        unary_method_impls!(private_api_request, $($func: $name,)*);
    };
}
