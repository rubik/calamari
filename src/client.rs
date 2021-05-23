use crate::models::BaseClient;

/// Base client class for the all the API clients. It shouldn't be used
/// directly, rather prefer the type synonyms [`KrakenPublicApiClient`],
/// [`KrakenPrivateApiClient`], etc.
pub struct ApiClient<C: BaseClient> {
    pub(crate) client: C,
}
