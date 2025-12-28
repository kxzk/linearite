use crate::types::{GraphQLRequest, GraphQLResponse};
use serde::Deserialize;
use serde_json::Value;
use std::env;

const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

pub fn get_api_key() -> Result<String, String> {
    env::var("LINEAR_API_KEY")
        .map_err(|_| "LINEAR_API_KEY environment variable not set".to_string())
}

#[derive(Deserialize)]
struct GraphQLErrorResponse {
    pub errors: Option<Vec<GraphQLError>>,
    #[serde(skip)]
    #[allow(dead_code)]
    pub data: Option<Value>,
}

#[derive(Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(default)]
    pub extensions: Option<Value>,
}

async fn query_linear_internal<T>(
    api_key: &str,
    query: &'static str,
    variables: Option<Value>,
    api_url: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let client = reqwest::Client::new();
    let request = GraphQLRequest { query, variables };

    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", api_key)
        .json(&request)
        .send()
        .await?;

    let text = response.text().await?;

    if let Ok(error_response) = serde_json::from_str::<GraphQLErrorResponse>(&text)
        && let Some(errors) = error_response.errors
    {
        let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
        return Err(format!("GraphQL errors: {}", error_messages.join(", ")).into());
    }

    let graphql_response: GraphQLResponse<T> = serde_json::from_str(&text)
        .map_err(|e| format!("Failed to parse response: {}. Response body: {}", e, text))?;

    Ok(graphql_response.data)
}

pub async fn query_linear<T>(
    api_key: &str,
    query: &'static str,
    variables: Option<Value>,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    query_linear_internal(api_key, query, variables, LINEAR_API_URL).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{IssueCreateResponse, ProjectsResponse, TeamsResponse};
    use serde_json::json;
    use std::sync::Mutex;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{header, method, path},
    };

    // Mutex to serialize environment variable access in tests
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    #[tokio::test]
    async fn test_query_linear_teams() {
        let mock_server = MockServer::start().await;

        let response_body = json!({
            "data": {
                "teams": {
                    "nodes": [
                        {"id": "team-1", "name": "Engineering"},
                        {"id": "team-2", "name": "Product"}
                    ]
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(header("Content-Type", "application/json"))
            .and(header("Authorization", "test-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: TeamsResponse = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await
        .unwrap();

        assert_eq!(result.teams.nodes.len(), 2);
        assert_eq!(result.teams.nodes[0].id, "team-1");
        assert_eq!(result.teams.nodes[0].name, "Engineering");
        assert_eq!(result.teams.nodes[1].id, "team-2");
        assert_eq!(result.teams.nodes[1].name, "Product");
    }

    #[tokio::test]
    async fn test_query_linear_projects() {
        let mock_server = MockServer::start().await;

        let response_body = json!({
            "data": {
                "projects": {
                    "nodes": [
                        {"id": "proj-1", "name": "Project Alpha"}
                    ]
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: ProjectsResponse = query_linear_internal(
            "test-key",
            "query Projects { projects { nodes { id name } } }",
            None,
            &url,
        )
        .await
        .unwrap();

        assert_eq!(result.projects.nodes.len(), 1);
        assert_eq!(result.projects.nodes[0].id, "proj-1");
        assert_eq!(result.projects.nodes[0].name, "Project Alpha");
    }

    #[tokio::test]
    async fn test_query_linear_create_issue() {
        let mock_server = MockServer::start().await;

        let response_body = json!({
            "data": {
                "issueCreate": {
                    "success": true,
                    "issue": {
                        "id": "issue-123",
                        "title": "Test Issue",
                        "url": "https://linear.app/issue-123"
                    }
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let variables = json!({
            "input": {
                "teamId": "team-123",
                "title": "Test Issue"
            }
        });

        let result: IssueCreateResponse = query_linear_internal(
            "test-key",
            "mutation IssueCreate($input: IssueCreateInput!) { issueCreate(input: $input) { success issue { id title url } } }",
            Some(variables),
            &url,
        )
        .await
        .unwrap();

        assert!(result.issue_create.success);
        assert!(result.issue_create.issue.is_some());
        let issue = result.issue_create.issue.unwrap();
        assert_eq!(issue.id, "issue-123");
        assert_eq!(issue.title, "Test Issue");
    }

    #[tokio::test]
    async fn test_query_linear_error_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_linear_graphql_errors_single() {
        let mock_server = MockServer::start().await;

        let error_response = json!({
            "errors": [
                {
                    "message": "Unauthorized",
                    "extensions": {
                        "code": "UNAUTHENTICATED"
                    }
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("GraphQL errors"));
            assert!(error_msg.contains("Unauthorized"));
        }
    }

    #[tokio::test]
    async fn test_query_linear_graphql_errors_multiple() {
        let mock_server = MockServer::start().await;

        let error_response = json!({
            "errors": [
                {
                    "message": "Error 1"
                },
                {
                    "message": "Error 2"
                },
                {
                    "message": "Error 3"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("GraphQL errors"));
            assert!(error_msg.contains("Error 1"));
            assert!(error_msg.contains("Error 2"));
            assert!(error_msg.contains("Error 3"));
        }
    }

    #[tokio::test]
    async fn test_query_linear_invalid_json_response() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            assert!(error_msg.contains("Failed to parse response"));
        }
    }

    #[tokio::test]
    async fn test_query_linear_missing_data_field() {
        let mock_server = MockServer::start().await;

        let invalid_response = json!({
            "not_data": {
                "teams": {
                    "nodes": []
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&invalid_response))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_linear_malformed_response_structure() {
        let mock_server = MockServer::start().await;

        let malformed_response = json!({
            "data": {
                "teams": {
                    "wrong_field": []
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&malformed_response))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "test-key",
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_get_api_key_success() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save original value if it exists
        let original_value = std::env::var("LINEAR_API_KEY").ok();

        // Set environment variable for test
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-api-key-123");
        }
        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-api-key-123");

        // Restore original value or remove if it didn't exist
        match original_value {
            Some(val) => unsafe {
                std::env::set_var("LINEAR_API_KEY", &val);
            },
            None => unsafe {
                std::env::remove_var("LINEAR_API_KEY");
            },
        }
    }

    #[test]
    fn test_get_api_key_missing() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save original value if it exists
        let original_value = std::env::var("LINEAR_API_KEY").ok();

        // Ensure variable is not set
        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }

        // Verify the variable is actually removed
        assert!(
            std::env::var("LINEAR_API_KEY").is_err(),
            "LINEAR_API_KEY should be removed before testing"
        );

        let result = get_api_key();
        assert!(result.is_err(), "get_api_key should fail when LINEAR_API_KEY is not set");
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("LINEAR_API_KEY"));
        assert!(error_msg.contains("not set"));

        // Restore original value if it existed
        if let Some(val) = original_value {
            unsafe {
                std::env::set_var("LINEAR_API_KEY", &val);
            }
        } else {
            // Double-check it's still removed
            unsafe {
                std::env::remove_var("LINEAR_API_KEY");
            }
        }
    }

    #[test]
    fn test_get_api_key_empty_string() {
        let _guard = ENV_MUTEX.lock().unwrap();

        // Save original value if it exists
        let original_value = std::env::var("LINEAR_API_KEY").ok();

        // Set empty string
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "");
        }
        let result = get_api_key();
        // Empty string is technically valid (env var exists but is empty)
        // The function should return Ok("") if that's what's set
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");

        // Restore original value or remove if it didn't exist
        match original_value {
            Some(val) => unsafe {
                std::env::set_var("LINEAR_API_KEY", &val);
            },
            None => unsafe {
                std::env::remove_var("LINEAR_API_KEY");
            },
        }
    }
}
