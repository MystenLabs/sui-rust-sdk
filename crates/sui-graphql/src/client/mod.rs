//! GraphQL client for Sui blockchain.

pub(crate) mod chain;
pub(crate) mod checkpoints;
pub(crate) mod coins;
pub(crate) mod dynamic_fields;
pub(crate) mod execution;
pub(crate) mod objects;
pub(crate) mod transactions;

use reqwest::Url;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;
use serde::Deserialize;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::error::GraphQLError;
use crate::response::Response;

/// Header the server reads into its `client_sdk_type` metric label. `rust` must stay in the
/// server's SDK-type allowlist to be tracked verbatim.
const CLIENT_SDK_TYPE_HEADER: HeaderName = HeaderName::from_static("client-sdk-type");

/// GraphQL client for Sui blockchain.
#[derive(Clone, Debug)]
pub struct Client {
    endpoint: Url,
    http: reqwest::Client,
    headers: HeaderMap,
}

impl Client {
    /// URL for the Sui Foundation provided GraphQL service for mainnet.
    pub const MAINNET: &str = "https://graphql.mainnet.sui.io/graphql";

    /// URL for the Sui Foundation provided GraphQL service for testnet.
    pub const TESTNET: &str = "https://graphql.testnet.sui.io/graphql";

    /// URL for the Sui Foundation provided GraphQL service for devnet.
    pub const DEVNET: &str = "https://graphql.devnet.sui.io/graphql";

    /// Create a new GraphQL client with the given endpoint.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sui_graphql::Client;
    ///
    /// let client = Client::new(Client::MAINNET).unwrap();
    /// ```
    pub fn new(endpoint: &str) -> Result<Self, Error> {
        let endpoint = Url::parse(endpoint)?;
        Ok(Self {
            endpoint,
            http: reqwest::Client::new(),
            headers: HeaderMap::new(),
        })
    }

    /// Replace the headers attached to every outgoing request with `headers`.
    ///
    /// Useful for authenticated GraphQL gateways (`Authorization`, `X-Api-Key`)
    /// or for forwarding tenant/trace metadata. See also [`Client::extend_headers`],
    /// [`Client::bearer_auth`], [`Client::basic_auth`].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sui_graphql::Client;
    /// use sui_graphql::header::HeaderMap;
    /// use sui_graphql::header::HeaderValue;
    ///
    /// let mut headers = HeaderMap::new();
    /// headers.insert("X-Api-Key", HeaderValue::from_static("my-key"));
    /// let client = Client::new(Client::MAINNET).unwrap().with_headers(headers);
    /// ```
    pub fn with_headers(mut self, headers: HeaderMap) -> Self {
        self.headers = headers;
        self
    }

    /// Merge the given headers into the client's outgoing-request header set.
    ///
    /// For every header name present in `headers`, all existing values for that
    /// name on the client are replaced with the values from `headers`. Header
    /// names not present in `headers` are left untouched. Within `headers`,
    /// multiple values for the same name are preserved (appended in order).
    ///
    /// Note: this differs from [`HeaderMap`]'s own `Extend` impl, which appends
    /// for every entry — that would let stale `Authorization` / `X-Api-Key`
    /// values linger alongside the new ones.
    pub fn extend_headers(&mut self, headers: HeaderMap) -> &mut Self {
        let mut current_key: Option<reqwest::header::HeaderName> = None;
        for (key, value) in headers {
            match key {
                Some(k) => {
                    self.headers.insert(k.clone(), value);
                    current_key = Some(k);
                }
                None => {
                    if let Some(k) = &current_key {
                        self.headers.append(k, value);
                    }
                }
            }
        }
        self
    }

    /// Set an `Authorization: Bearer {token}` header on every outgoing request,
    /// marking the value as sensitive (will not be logged by `reqwest`).
    ///
    /// Mirrors [`sui_rpc::client::HeadersInterceptor::bearer_auth`] for
    /// cross-transport API consistency.
    pub fn bearer_auth<T>(&mut self, token: T) -> &mut Self
    where
        T: std::fmt::Display,
    {
        let value = format!("Bearer {token}");
        let mut header = reqwest::header::HeaderValue::from_str(&value)
            .expect("token is always a valid HeaderValue");
        header.set_sensitive(true);
        self.headers.insert(reqwest::header::AUTHORIZATION, header);
        self
    }

    /// Set an `Authorization: Basic <base64(user:pass)>` header on every
    /// outgoing request, marking the value as sensitive.
    ///
    /// Mirrors [`sui_rpc::client::HeadersInterceptor::basic_auth`] for
    /// cross-transport API consistency.
    pub fn basic_auth<U, P>(&mut self, username: U, password: Option<P>) -> &mut Self
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        use base64ct::Base64;
        use base64ct::Encoding;
        let pair = match password {
            Some(p) => format!("{username}:{p}"),
            None => format!("{username}:"),
        };
        let value = format!("Basic {}", Base64::encode_string(pair.as_bytes()));
        let mut header = reqwest::header::HeaderValue::from_str(&value)
            .expect("base64 is always a valid HeaderValue");
        header.set_sensitive(true);
        self.headers.insert(reqwest::header::AUTHORIZATION, header);
        self
    }

    /// Execute a GraphQL query and return the response.
    ///
    /// The response contains both data and any errors (GraphQL supports partial success).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use serde::Deserialize;
    /// use sui_graphql::Client;
    ///
    /// #[derive(Deserialize)]
    /// struct MyResponse {
    ///     #[serde(rename = "chainIdentifier")]
    ///     chain_identifier: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), sui_graphql::Error> {
    ///     let client = Client::new(Client::MAINNET)?;
    ///     let response = client
    ///         .query::<MyResponse>("query { chainIdentifier }", serde_json::json!({}))
    ///         .await?;
    ///
    ///     // Check for partial errors
    ///     if response.has_errors() {
    ///         for err in response.errors() {
    ///             eprintln!("GraphQL error: {}", err.message());
    ///         }
    ///     }
    ///
    ///     // Access the data
    ///     if let Some(data) = response.data() {
    ///         println!("Chain: {}", data.chain_identifier);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn query<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> Result<Response<T>, Error> {
        #[derive(Serialize)]
        struct GraphQLRequest<'a> {
            query: &'a str,
            variables: serde_json::Value,
        }

        #[derive(Deserialize)]
        struct GraphQLResponse<T> {
            data: Option<T>,
            errors: Option<Vec<GraphQLError>>,
        }

        let request = GraphQLRequest { query, variables };

        let mut headers = self.headers.clone();
        headers.insert(CLIENT_SDK_TYPE_HEADER, HeaderValue::from_static("rust"));

        let req = self
            .http
            .post(self.endpoint.clone())
            .json(&request)
            .headers(headers);
        let resp = req.send().await?;
        let status = resp.status();

        if status.is_success() {
            let raw: GraphQLResponse<T> = resp.json().await?;
            return Ok(Response::new(raw.data, raw.errors.unwrap_or_default()));
        }

        // A non-success status carries a GraphQL response only when the body holds GraphQL
        // content. Anything else came from an intermediary rather than from the GraphQL server,
        // including JSON that deserializes into an empty response, which would otherwise be
        // indistinguishable from a query that returned no data.
        let body = resp.bytes().await?;
        match serde_json::from_slice::<GraphQLResponse<T>>(&body) {
            Ok(raw) if raw.data.is_some() || raw.errors.is_some() => {
                Ok(Response::new(raw.data, raw.errors.unwrap_or_default()))
            }
            _ => Err(Error::HttpStatus {
                status: status.as_u16(),
                body: body_snippet(&body),
            }),
        }
    }
}

/// The start of a response body, short enough to keep an HTML error page out of a log line.
fn body_snippet(body: &[u8]) -> String {
    String::from_utf8_lossy(body).chars().take(500).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::header::HeaderValue;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::header;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[test]
    fn test_client_new() {
        let client = Client::new("https://example.com/graphql").unwrap();
        assert_eq!(client.endpoint.as_str(), "https://example.com/graphql");
        assert!(client.headers.is_empty());
    }

    #[test]
    fn test_client_new_invalid_url() {
        let result = Client::new("not a valid url");
        assert!(matches!(result, Err(Error::InvalidUrl(_))));
    }

    fn ok_body() -> serde_json::Value {
        serde_json::json!({"data": {"chainIdentifier": "test"}, "errors": null})
    }

    #[derive(Debug, Deserialize)]
    struct Chain {
        #[serde(rename = "chainIdentifier")]
        chain: String,
    }

    /// Answer a single query with `template` and hand back whatever the client made of it.
    async fn query_against(template: ResponseTemplate) -> Result<Response<Chain>, Error> {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(template)
            .mount(&server)
            .await;

        Client::new(&server.uri())
            .unwrap()
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
    }

    #[tokio::test]
    async fn with_headers_round_trip() {
        let server = MockServer::start().await;
        let mut headers = HeaderMap::new();
        headers.insert("X-Api-Key", HeaderValue::from_static("kcolb"));
        headers.insert("X-Tenant", HeaderValue::from_static("monsoon"));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("x-api-key", "kcolb"))
            .and(header("x-tenant", "monsoon"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::new(&server.uri()).unwrap().with_headers(headers);
        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn bearer_auth_round_trip() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("authorization", "Bearer s3cr3t"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let mut client = Client::new(&server.uri()).unwrap();
        client.bearer_auth("s3cr3t");
        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn basic_auth_round_trip() {
        // base64("alice:hunter2") == "YWxpY2U6aHVudGVyMg=="
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("authorization", "Basic YWxpY2U6aHVudGVyMg=="))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let mut client = Client::new(&server.uri()).unwrap();
        client.basic_auth("alice", Some("hunter2"));
        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn extend_headers_overwrites_existing_keys() {
        // Start with X-Api-Key=stale + X-Tenant=monsoon. Extend with
        // X-Api-Key=fresh and X-Trace=abc — the result must be:
        //   X-Api-Key: fresh        (stale value gone, not appended alongside)
        //   X-Tenant:  monsoon      (untouched)
        //   X-Trace:   abc          (new entry)
        let server = MockServer::start().await;
        let mut initial = HeaderMap::new();
        initial.insert("X-Api-Key", HeaderValue::from_static("stale"));
        initial.insert("X-Tenant", HeaderValue::from_static("monsoon"));

        let mut overlay = HeaderMap::new();
        overlay.insert("X-Api-Key", HeaderValue::from_static("fresh"));
        overlay.insert("X-Trace", HeaderValue::from_static("abc"));

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("x-api-key", "fresh"))
            .and(header("x-tenant", "monsoon"))
            .and(header("x-trace", "abc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let mut client = Client::new(&server.uri()).unwrap().with_headers(initial);
        client.extend_headers(overlay);

        // Sanity-check the in-memory map: X-Api-Key has exactly one value,
        // not two. (wiremock's `header(...)` matcher only checks presence,
        // so this guards against silent multi-value duplication.)
        assert_eq!(client.headers.get_all("X-Api-Key").iter().count(), 1);
        assert_eq!(
            client.headers.get("X-Api-Key").unwrap(),
            &HeaderValue::from_static("fresh")
        );

        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn sdk_headers_forced_on_every_request() {
        // Even a bare client must advertise its SDK type, and a caller attempting to override it
        // must lose: the SDK forces its own value so server-side metrics attribute traffic to
        // this crate.
        let server = MockServer::start().await;
        let mut spoofed = HeaderMap::new();
        spoofed.insert(
            CLIENT_SDK_TYPE_HEADER,
            HeaderValue::from_static("typescript"),
        );

        Mock::given(method("POST"))
            .and(path("/"))
            .and(header("client-sdk-type", "rust"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::new(&server.uri()).unwrap().with_headers(spoofed);
        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn empty_headers_no_op() {
        // The "absence" path: a default Client with no custom headers must
        // still send a valid request. wiremock will reject unmatched requests,
        // so the bare `method=POST, path=/` matcher proves no auth/headers are
        // accidentally injected.
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(ok_body()))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::new(&server.uri()).unwrap();
        let _: Response<Chain> = client
            .query("query { chainIdentifier }", serde_json::json!({}))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn success_returns_data() {
        let response = query_against(ResponseTemplate::new(200).set_body_json(ok_body()))
            .await
            .unwrap();
        assert_eq!(response.data().unwrap().chain, "test");
    }

    #[tokio::test]
    async fn gateway_html_reports_status() {
        let template =
            ResponseTemplate::new(503).set_body_string("<html>Service Unavailable</html>");

        match query_against(template).await {
            Err(Error::HttpStatus { status, body }) => {
                assert_eq!(status, 503);
                assert!(body.contains("Service Unavailable"), "body: {body}");
            }
            other => panic!("expected HttpStatus, got: {other:?}"),
        }
    }

    /// An unrelated JSON body deserializes into an empty `GraphQLResponse`, so without the status
    /// check it is indistinguishable from a query that returned no data.
    #[tokio::test]
    async fn unrelated_json_reports_status() {
        let template = ResponseTemplate::new(401)
            .set_body_string(r#"{"error":"UNAUTHORIZED","request-id":""}"#);

        match query_against(template).await {
            Err(Error::HttpStatus { status, body }) => {
                assert_eq!(status, 401);
                assert!(body.contains("UNAUTHORIZED"), "body: {body}");
            }
            other => panic!("expected HttpStatus, got: {other:?}"),
        }
    }

    #[tokio::test]
    async fn empty_body_reports_status() {
        match query_against(ResponseTemplate::new(502).set_body_string("")).await {
            Err(Error::HttpStatus { status, .. }) => assert_eq!(status, 502),
            other => panic!("expected HttpStatus, got: {other:?}"),
        }
    }

    /// GraphQL servers may report query-level errors with a non-success status. Those are real
    /// GraphQL responses and stay available to the caller.
    #[tokio::test]
    async fn graphql_errors_survive_non_success_status() {
        let template = ResponseTemplate::new(400).set_body_string(
            r#"{"errors":[{"message":"Unknown field \"foo\" on type \"Query\"."}]}"#,
        );

        let response = query_against(template).await.unwrap();
        assert!(response.has_errors());
        assert_eq!(
            response.errors()[0].message(),
            "Unknown field \"foo\" on type \"Query\"."
        );
    }

    /// Success responses are decoded exactly as before, including how they fail.
    #[tokio::test]
    async fn success_with_undecodable_body_stays_a_decode_error() {
        let template = ResponseTemplate::new(200).set_body_string("not json at all");

        match query_against(template).await {
            Err(Error::Request(err)) => assert!(err.is_decode(), "err: {err}"),
            other => panic!("expected a decode error, got: {other:?}"),
        }
    }
}
