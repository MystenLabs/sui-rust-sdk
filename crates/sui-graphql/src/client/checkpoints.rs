//! Checkpoint-related convenience methods.

use sui_graphql_macros::Response;
use sui_sdk_types::CheckpointContents;
use sui_sdk_types::CheckpointSummary;

use super::Client;
use crate::bcs::Bcs;
use crate::error::Error;

/// A checkpoint response containing the summary and contents.
///
/// This struct combines the checkpoint header (summary) with its contents.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CheckpointResponse {
    /// The checkpoint summary (epoch, sequence number, timestamp, etc.)
    pub summary: CheckpointSummary,
    /// The checkpoint contents (transaction digests and signatures)
    pub contents: CheckpointContents,
}

impl Client {
    /// Fetch a checkpoint by its sequence number, or the latest checkpoint if not specified.
    ///
    /// Returns:
    /// - `Ok(Some(response))` if the checkpoint exists
    /// - `Ok(None)` if the checkpoint does not exist
    /// - `Err(Error::Request)` for network errors
    /// - `Err(Error::Base64)` / `Err(Error::Bcs)` for decoding errors
    pub async fn get_checkpoint(
        &self,
        sequence_number: Option<u64>,
    ) -> Result<Option<CheckpointResponse>, Error> {
        #[derive(Response)]
        struct Response {
            #[field(path = "checkpoint.summaryBcs")]
            summary_bcs: Option<Bcs<CheckpointSummary>>,
            #[field(path = "checkpoint.contentBcs")]
            content_bcs: Option<Bcs<CheckpointContents>>,
        }

        const QUERY: &str = r#"
            query($sequenceNumber: UInt53) {
                checkpoint(sequenceNumber: $sequenceNumber) {
                    summaryBcs
                    contentBcs
                }
            }
        "#;
        let variables = serde_json::json!({ "sequenceNumber": sequence_number });

        let response = self.query::<Response>(QUERY, variables).await?;

        let Some(data) = response.into_data() else {
            return Ok(None);
        };

        let (Some(summary), Some(contents)) = (data.summary_bcs, data.content_bcs) else {
            return Ok(None);
        };

        Ok(Some(CheckpointResponse {
            summary: summary.0,
            contents: contents.0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    #[tokio::test]
    async fn test_get_checkpoint_not_found() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": {
                    "checkpoint": null
                }
            })))
            .mount(&mock_server)
            .await;

        let client = Client::new(&mock_server.uri()).unwrap();

        let result = client.get_checkpoint(Some(999999999)).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
