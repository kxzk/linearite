use crate::types::{GraphQLRequest, GraphQLResponse};
use serde::Deserialize;
use serde_json::Value;
use std::borrow::Cow;
use std::env;

const LINEAR_API_URL: &str = "https://api.linear.app/graphql";

pub fn get_api_key() -> Result<String, String> {
    env::var("LINEAR_API_KEY").map_err(|_| "LINEAR_API_KEY environment variable not set".to_string())
}

async fn query_linear_internal<T>(
    query: &str,
    variables: Option<Value>,
    api_url: &str,
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let api_key = get_api_key()?;
    
    let client = reqwest::Client::new();
    let request = GraphQLRequest {
        query: Cow::Borrowed(query),
        variables,
    };
    
    let response = client
        .post(api_url)
        .header("Content-Type", "application/json")
        .header("Authorization", api_key)
        .json(&request)
        .send()
        .await?;

    let graphql_response: GraphQLResponse<T> = response.json().await?;
    Ok(graphql_response.data)
}

pub async fn query_linear<T>(query: &str, variables: Option<Value>) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de>,
{
    query_linear_internal(query, variables, LINEAR_API_URL).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{IssueCreateResponse, ProjectsResponse, TeamsResponse};
    use serde_json::json;
    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, ResponseTemplate,
    };

    #[test]
    fn test_get_api_key_success() {
        // Set the variable right before checking to avoid race conditions
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key-123");
        }
        let result = get_api_key();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-key-123");
        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
    }

    #[test]
    fn test_get_api_key_failure() {
        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
        let result = get_api_key();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("LINEAR_API_KEY"));
    }

    #[tokio::test]
    async fn test_query_linear_teams() {
        let mock_server = MockServer::start().await;
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }

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

        // Ensure API key is set right before the call
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }
        
        let url = format!("{}/graphql", mock_server.uri());
        let result: TeamsResponse = query_linear_internal(
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

        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
    }

    #[tokio::test]
    async fn test_query_linear_projects() {
        let mock_server = MockServer::start().await;
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }

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

        // Ensure API key is set right before the call
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }
        
        let url = format!("{}/graphql", mock_server.uri());
        let result: ProjectsResponse = query_linear_internal(
            "query Projects { projects { nodes { id name } } }",
            None,
            &url,
        )
        .await
        .unwrap();

        assert_eq!(result.projects.nodes.len(), 1);
        assert_eq!(result.projects.nodes[0].id, "proj-1");
        assert_eq!(result.projects.nodes[0].name, "Project Alpha");

        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
    }

    #[tokio::test]
    async fn test_query_linear_create_issue() {
        let mock_server = MockServer::start().await;
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }

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

        // Ensure API key is set right before the call
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }
        
        let url = format!("{}/graphql", mock_server.uri());
        let variables = json!({
            "input": {
                "teamId": "team-123",
                "title": "Test Issue"
            }
        });

        let result: IssueCreateResponse = query_linear_internal(
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

        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
    }

    #[tokio::test]
    async fn test_query_linear_error_response() {
        let mock_server = MockServer::start().await;
        unsafe {
            std::env::set_var("LINEAR_API_KEY", "test-key");
        }

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());

        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }
    }

    #[tokio::test]
    async fn test_query_linear_missing_api_key() {
        let mock_server = MockServer::start().await;
        
        // Ensure variable is removed right before the call
        unsafe {
            std::env::remove_var("LINEAR_API_KEY");
        }

        let url = format!("{}/graphql", mock_server.uri());
        let result: Result<TeamsResponse, _> = query_linear_internal(
            "query Teams { teams { nodes { id name } } }",
            None,
            &url,
        )
        .await;

        assert!(result.is_err());
        if let Err(e) = result {
            let error_msg = format!("{}", e);
            // The error should mention LINEAR_API_KEY
            assert!(error_msg.contains("LINEAR_API_KEY") || error_msg.contains("environment variable"));
        }
    }
}

